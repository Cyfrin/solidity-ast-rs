use foundry_compilers::{
    ProjectPathsConfig,
    artifacts::EvmVersion,
    solc::{Solc, SolcLanguage},
};
use foundry_rs_config::filter::GlobMatcher;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ProjectConfigInput {
    /// Root directory must contain hardhat.config.ts/.js or foundry.toml or (it's FOUNDRY_
    /// equivalet name)
    pub root: PathBuf,

    /// Project Paths representation (contains sources)
    pub project_paths: ProjectPathsConfig<SolcLanguage>,

    /// Paths (sources) containing these strings will be included
    pub include_containing: Vec<String>,

    /// Paths (sources) containing these strings will be excluded
    pub exclude_containing: Vec<String>,

    /// List of Absolute Paths (nested in sources) that will be excluded
    pub exclude_starting: Vec<PathBuf>,

    /// Paths (sources) matching with these will be excluded
    pub skip: Vec<GlobMatcher>,

    /// Solc Version to use for compiling
    pub solc_compiler: SolcCompilerConfigInput,

    /// Evm Version
    pub evm_version: EvmVersion,

    /// Whether via_ir is set to true in foundry.toml for the profile
    pub via_ir: bool,
}

#[derive(Debug)]
pub enum SolcCompilerConfigInput {
    /// Use graphs to resolve versions and group files
    AutoDetect,

    /// Use specific version
    Specific(Solc),
}
