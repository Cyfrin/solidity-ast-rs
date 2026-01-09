use super::{ProjectConfigInput, SolcCompilerConfigInput, VersionedAstOutputs};
use crate::{Result, SolcCompilerOutput, project_config::command::compile_output};
use foundry_compilers::{
    Graph, ProjectBuilder,
    artifacts::{
        Settings, SolcInput, Source, Sources, StandardJsonCompilerInput,
        output_selection::OutputSelection,
    },
    resolver::parse::SolParser,
    solc::{Solc, SolcCompiler, SolcLanguage},
    utils,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use semver::Version;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

impl ProjectConfigInput {
    /// Generate input material for solc compiler to retrieve ASTs
    pub fn solc_input_for_ast_generation(&self) -> Result<HashMap<semver::Version, SolcInput>> {
        let create_standard_json_for_ast = |sources: Sources, version: &Version| -> SolcInput {
            let mut settings = Settings::new(OutputSelection::ast_output_selection());
            settings.remappings = self.project_paths.remappings.clone();
            settings.sanitize(version, SolcLanguage::Solidity);
            let root = utils::canonicalize(self.root.clone()).expect("root failed to canonicalize");
            let mut solc_input = SolcInput::new(SolcLanguage::Solidity, sources, settings);
            solc_input.strip_prefix(&root);
            solc_input
        };

        let sources = Source::read_all_files(
            source_files_iter(&self.project_paths.sources, &self).collect(),
        )?;

        match &self.solc_compiler {
            SolcCompilerConfigInput::AutoDetect => {
                let project = ProjectBuilder::<SolcCompiler>::default()
                    .paths(self.project_paths.clone()) // cheap enough to clone (doesn't contain content)
                    .build(Default::default())?;

                let solidity_sources = {
                    let graph = Graph::<SolParser>::resolve_sources(&self.project_paths, sources)?;
                    graph.into_sources_by_version(&project)?.sources.remove(&SolcLanguage::Solidity)
                };

                let Some(versioned_sources) = solidity_sources else {
                    return Ok(Default::default()); // expect no *.sol files
                };

                // ensure all required solc versions are nstalled
                for (v, _, _) in &versioned_sources {
                    Solc::find_or_install(v)?;
                }

                Ok(HashMap::from_iter(
                    versioned_sources
                        .into_iter()
                        .map(|(v, s, _)| (v.clone(), create_standard_json_for_ast(s, &v))),
                ))
            }

            SolcCompilerConfigInput::Specific(solc) => {
                let graph = Graph::<SolParser>::resolve_sources(&self.project_paths, sources)?;
                let (sources, _) = graph.into_sources();

                let versioned_sources = HashMap::from_iter(vec![(
                    solc.version.clone(),
                    create_standard_json_for_ast(sources, &solc.version),
                )]);
                Ok(versioned_sources)
            }
        }
    }

    pub fn standard_json_for_ast_generation(
        &self,
    ) -> Result<HashMap<semver::Version, StandardJsonCompilerInput>> {
        let solc_input = self.solc_input_for_ast_generation()?;
        Ok(solc_input.into_iter().map(|(k, v)| (k, v.into())).collect())
    }

    pub fn make_asts(&self) -> Result<Vec<VersionedAstOutputs>> {
        let make_ast = |version: Version, solc_input: SolcInput| -> Result<VersionedAstOutputs> {
            // Grab the relevant solc compiler
            let mut solc = Solc::find_or_install(&version)?;

            // Include and allow paths may be extra parameters mentioned in foundry.toml which we
            // proxy to solc
            // solc.include_paths = self.project_paths.include_paths.clone();
            // println!("SIP, SAP: {:?} {:?}", solc.include_paths, solc.allow_paths);
            solc.allow_paths.insert(self.root.clone());
            solc.allow_paths.insert("/".into());

            // Retrieve the ASTs
            let output = compile_output(&solc, self.root.as_path(), &solc_input)?;

            let str_output = std::str::from_utf8(&output)?;
            let compiler_output: SolcCompilerOutput = serde_json::from_str(str_output)?;

            // Tag is_included
            let included_files =
                compiler_output.sources.keys().filter(|k| self.is_included(k)).cloned().collect();

            // Versioned Ast
            Ok(VersionedAstOutputs {
                version: version.clone(),
                compiler_output,
                included_files,
                sources: solc_input.sources,
            })
        };

        let mut asts: Vec<VersionedAstOutputs> = Default::default();

        let ast_results: Vec<_> = self
            .solc_input_for_ast_generation()?
            .into_par_iter()
            .map(|(v, s)| make_ast(v, s))
            .collect();

        for ast_result in ast_results {
            let ast = ast_result?;
            asts.push(ast);
        }

        Ok(asts)
    }

    /// Returns if a file should be included
    pub(crate) fn is_included(&self, path: &Path) -> bool {
        let root = utils::canonicalize(self.root.clone()).expect("root failed to canonicalize");

        // Construct the full path
        let path = root.join(path);

        // Inside the src directory
        if path.strip_prefix(self.project_paths.sources.clone()).is_err() {
            return false;
        }

        // Auto exclude
        if self.exclude_starting.iter().any(|exclude| path.starts_with(exclude)) {
            return false;
        }

        // skip (value in foundry.toml)
        if self.skip.iter().any(|skipper| skipper.is_match(&path)) {
            return false;
        }

        let path_str = path
            .strip_prefix(root)
            .expect("failed to strip prefix from root")
            .as_os_str()
            .to_string_lossy();

        // Exclude containing
        if self.exclude_containing.iter().any(|exclude_string| path_str.contains(exclude_string)) {
            return false;
        }

        // Include containing
        self.include_containing.iter().any(|include_string| path_str.contains(include_string))
    }
}

pub fn source_files_iter(src: &Path, config: &ProjectConfigInput) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(src)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| {
            // skip from foundry.toml config
            config.skip.iter().all(|skipper| !skipper.is_match(e.path())) &&

            // don't descend into directories that are meant to be excluded
            config.exclude_starting.iter().all(|exclude| !e.path().starts_with(exclude))
        })
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|ext| ext == "sol").unwrap_or_default())
        .filter(|e| config.is_included(e.path())) // Final filter
        .map(|e| e.path().into())
}
