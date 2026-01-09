use foundry_compilers::artifacts::{EvmVersion, Sources};
use semver::Version;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DerivedAstEvmInfo {
    /// Groups of AST outputs, each is it's own context
    pub versioned_asts: Vec<VersionedAstOutputs>,
    /// EVM version derived from configuration file
    pub evm_version: EvmVersion,
    /// Whether via_ir is set to to true in foundry.toml
    pub via_ir: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct VersionedAstOutputs {
    /// Solidity version used to compile this group of Solidity files
    pub version: Version,
    /// Deserialized output of running `solc` command
    pub compiler_output: SolcCompilerOutput,
    /// Set of files that pass the include and exclude tests/arguments
    /// supplied when creating [`super::ProjectConfigInput`]
    pub included_files: HashSet<PathBuf>,
    /// Key Value store contains the content for all Paths in context
    pub sources: Sources,
}

/// Output type `solc` produces
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
pub struct SolcCompilerOutput {
    #[serde(default)]
    pub errors: Vec<foundry_compilers::artifacts::Error>,
    #[serde(default)]
    pub sources: BTreeMap<PathBuf, AstSourceFile>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct AstSourceFile {
    pub id: u32,
    #[serde(deserialize_with = "raw_map_string::deserialize")]
    pub ast: Option<String>,
}

mod raw_map_string {
    use serde::{Deserialize, Deserializer};
    use serde_json::{self, Value};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Value::deserialize(deserializer)?;
        if value.is_null() {
            return Ok(None);
        }
        let string = serde_json::to_string(&value).map_err(serde::de::Error::custom)?;
        Ok(Some(string))
    }
}
