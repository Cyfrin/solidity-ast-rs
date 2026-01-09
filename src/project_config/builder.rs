use super::{ProjectConfigInput, SolcCompilerConfigInput};
use crate::Result;
use foundry_compilers::{
    solc::{Solc, SolcCompiler},
    utils,
};
use foundry_rs_config::Config;
use semver::Version;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

/// ## Builder for [`ProjectConfigInput`]
///
/// Assumes defaults based on commonly used frameworks like
/// Foundry, Hardhat, Soldeer also allowing you to speccify custom values with builder pattern
///
/// ### Note
///
/// Some files are automatically excluded *always* like tests, lib, dependencies, node_modules,
/// etc based on the framework.
pub struct ProjectConfigInputBuilder {
    root: PathBuf,
    sources: SourcesConfig,
    exclude: ExcludeConfig,
    include: IncludeConfig,
    solc_version: SolcVersionConfig,
}

#[derive(Default)]
pub enum SolcVersionConfig {
    /// Resolves sources by version either by graphs or reads from framework config.
    /// If the framework config is set to specific version of Solc, it will be respected
    #[default]
    Auto,

    /// Force override [`SolcVersionConfig::Auto`]
    Specific(Version),
}

/// Match paths of solidity files to be scanned
#[derive(Default)]
pub enum SourcesConfig {
    /// Resolve based on the framework or same as root in case of custom project
    #[default]
    AutoDetect,

    /// Path to a directory relative to the root containing Solidity contracts
    Specific(PathBuf),
}

/// Match paths of solidity files to be excluded
#[derive(Default)]
pub enum ExcludeConfig {
    /// Does not exclude any file in [`SourcesConfig`].
    #[default]
    None,

    /// Path segments of files to be excluded in addition to the automatically excluded files
    Specific(Vec<String>),
}

/// Match paths of solidity files to be included
#[derive(Default)]
pub enum IncludeConfig {
    /// Includes all files in [`SourcesConfig`] other than automatically excluded files.
    #[default]
    All,

    /// Path segments of files to be included as a subset of [`SourcesConfig`]
    Specific(Vec<String>),
}

pub(crate) const EMPTY_STRING: &str = "";

impl ProjectConfigInputBuilder {
    /// Creates a new instance of [`ProjectConfigInputBuilder`]
    /// Root directory must contain hardhat.config.ts/.js or foundry.toml or (it's FOUNDRY_
    /// equivalet name)
    pub fn new(root: &Path) -> ProjectConfigInputBuilder {
        ProjectConfigInputBuilder {
            root: root.to_owned(),
            sources: Default::default(),
            exclude: Default::default(),
            include: Default::default(),
            solc_version: Default::default(),
        }
    }
    pub fn with_sources(mut self, sources: SourcesConfig) -> ProjectConfigInputBuilder {
        self.sources = sources;
        self
    }
    pub fn with_exclude(mut self, exclude: ExcludeConfig) -> ProjectConfigInputBuilder {
        self.exclude = exclude;
        self
    }
    pub fn with_include(mut self, include: IncludeConfig) -> ProjectConfigInputBuilder {
        self.include = include;
        self
    }
    pub fn with_solc_version(mut self, version: SolcVersionConfig) -> ProjectConfigInputBuilder {
        self.solc_version = version;
        self
    }
    pub fn build(self) -> Result<ProjectConfigInput> {
        if !self.root.exists() {
            return Err(String::from("Non existent root").into());
        }

        let config = {
            let mut c = load_baseline_config(self.root.clone())?;
            // Sources
            if let SourcesConfig::Specific(src) = self.sources {
                c.src = src;
            }
            // Override offline so that config.ensure_solc() can download solc.
            c.offline = false;
            c.sanitized()
        };

        // Auto excludes
        let exclude_starting = {
            let mut e = Vec::new();

            // sometimes, lib, test, script may point to a directory inside the src
            // that's when this helps otherwise there's no point since we don't look
            // outside src to begin with.
            let mut unwanted = config
                .libs
                .iter()
                .filter(|lib| lib.starts_with(&config.src))
                .cloned()
                .collect::<Vec<_>>();
            e.append(&mut unwanted);

            if config.test.starts_with(&config.src) {
                e.push(config.test.clone());
            }

            if config.script.starts_with(&config.src) {
                e.push(config.script.clone());
            }

            // dependencies must also be auto excluded if it's a soldeer project
            let soldeer_lock = self.root.join("soldeer.lock");
            let deps = self.root.join("dependencies");

            if soldeer_lock.is_file() && deps.is_dir() {
                let deps_norm = utils::canonicalize(deps)?;
                if deps_norm.starts_with(&config.src) {
                    e.push(deps_norm);
                }
            }

            // For whatever reason, if node_modules is used in custom project, ignore that
            let node_modules = self.root.join("node_modules");
            if node_modules.is_dir() {
                let node_modules_norm = utils::canonicalize(node_modules)?;
                if node_modules_norm.starts_with(&config.src) {
                    e.push(node_modules_norm);
                }
            }

            // New versions od hardhat include forge-std directly inside contracts/
            let forge_std = self.root.join("contracts").join("forge-std");
            if forge_std.is_dir() {
                let forge_std_norm = utils::canonicalize(forge_std)?;
                if forge_std_norm.starts_with(&config.src) {
                    e.push(forge_std_norm);
                }
            }

            e.dedup();
            e
        };

        // Excludes
        let exclude_containing = {
            match self.exclude {
                ExcludeConfig::None => Default::default(),
                ExcludeConfig::Specific(x) => x,
            }
        };

        // Includes
        let include_containing = {
            match self.include {
                IncludeConfig::All => vec![EMPTY_STRING.to_string()],
                IncludeConfig::Specific(x) => x,
            }
        };

        // Solc Compiler
        let solc_compiler = match &self.solc_version {
            SolcVersionConfig::Specific(solc_version) => {
                let solc = Solc::find_or_install(solc_version)?;
                SolcCompilerConfigInput::Specific(solc)
            }
            SolcVersionConfig::Auto => match config.solc_compiler()? {
                SolcCompiler::AutoDetect => SolcCompilerConfigInput::AutoDetect,
                SolcCompiler::Specific(solc) => SolcCompilerConfigInput::Specific(solc),
            },
        };

        Ok(ProjectConfigInput {
            project_paths: config.project_paths(),
            root: self.root,
            include_containing,
            exclude_containing,
            exclude_starting,
            skip: config.skip,
            solc_compiler,
            evm_version: config.evm_version,
            via_ir: config.via_ir,
        })
    }
}

fn load_baseline_config(root: PathBuf) -> Result<Config> {
    // Load config with auto detect default values
    let mut config = Config::load_with_root(&root)?;

    // If not HH/Foundry, reset config.src to self.root instead of `src`
    let hh_js = root.join("hardhat.config.js");
    let hh_ts = root.join("hardhat.config.ts");
    let foundry = root.join("foundry.toml");
    if !foundry.exists() && !hh_js.exists() && !hh_ts.exists() {
        // by default, if framework is not detected, it will be 'src'
        // we want it to be the same as root
        config.src = ".".into();

        let mut solidity_file_found_in_root = false;

        for file in std::fs::read_dir(&root)? {
            let file_path = file?.path();
            if file_path.extension().is_some_and(|e| e == OsStr::new("sol")) {
                solidity_file_found_in_root = true;
            }
        }

        if !solidity_file_found_in_root {
            let contracts = root.join("contracts");
            let src = root.join("src");

            if src.is_dir() {
                config.src = "src".into();
            } else if contracts.is_dir() {
                config.src = "contracts".into();
            }
        }
    }
    Ok(config)
}

#[cfg(test)]
mod tests {
    use crate::EMPTY_STRING;

    #[test]
    fn empty_string_is_always_a_substring() {
        let test_path = std::env::current_dir().unwrap();
        assert!(!test_path.as_os_str().is_empty());
        assert!(test_path.as_os_str().to_string_lossy().contains(EMPTY_STRING));
    }
}
