//! # Easily generate ASTs for Solidity Projects
//!
//! Derive ASTs and other Evm Info with builder API that allows passing information in increments.
//!
//! ## Goal
//!
//! Fastest way to generate ASTs from the `solc` compiler
//!
//! ## Example
//!
//!```no_run
//! use solidity_ast::{
//!     DerivedAstEvmInfo, ProjectConfigInputBuilder, Result, derive_ast_and_evm_info,
//! };
//! use std::path::Path;
//!
//! pub fn ast_info(root: &str) -> Result<DerivedAstEvmInfo> {
//!     let config = ProjectConfigInputBuilder::new(Path::new(root)).build()?;
//!     derive_ast_and_evm_info(&config)
//! }
//! ```

mod error;
mod project_config;

pub use error::*;
pub use foundry_compilers::artifacts::{EvmVersion, Source};
pub use project_config::*;

/// Returns AST and EVM based info based on foundry/hardhat's configuration (or) custom framework.
/// Use [`ProjectConfigInputBuilder::build`] to create the `config` argument for this function.
pub fn derive_ast_and_evm_info(config: &ProjectConfigInput) -> Result<DerivedAstEvmInfo> {
    let asts = config.make_asts()?;
    let evm_version = config.evm_version;
    let via_ir = config.via_ir;
    Ok(DerivedAstEvmInfo { versioned_asts: asts, evm_version, via_ir })
}
