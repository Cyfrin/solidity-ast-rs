# Solidity AST Generator

## Overview

To simplify installation, add the following to `Cargo.toml`

```toml
[dependencies]
solidity-ast-rs = { git = "https://github.com/Cyfrin/solidity-ast-rs", branch = "main", package = "solidity-ast" }

```

Extract ASTs and other Evm Info

```rust
use std::path::Path;
use solidity_ast_rs::{
    DerivedAstEvmInfo, ProjectConfigInputBuilder, Result, derive_ast_and_evm_info,
};

pub fn ast_info(root: &str) -> Result<DerivedAstEvmInfo> {
    let config = ProjectConfigInputBuilder::new(Path::new(root)).build()?;
    derive_ast_and_evm_info(&config)
}
```

## Goal

To be the fastest AST generator for [Aderyn](https://github.com/cyfrin/aderyn)

## Maintenance

Checkout the detailed steps to upgrade foundry dependencies documented in **[CONTRIBUTING.md](https://github.com/Cyfrin/solidity-ast-rs/blob/main/CONTRIBUTING.md)**

## Credits

This project exists thanks to all the people who [contribute](/CONTRIBUTING.md).<br>

<a href="https://github.com/cyfrin/foundry-compilers-aletheia/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=cyfrin/foundry-compilers-aletheia" />
</a>

## Attribution

#### [`foundry-compilers`](https://github.com/foundry-rs/compilers) 
#### [`foundry-config-rs`](https://github.com/foundry-rs/foundry) 

