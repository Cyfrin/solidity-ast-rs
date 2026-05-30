use pretty_assertions::assert_eq;

#[allow(unused_imports)]
mod common {
    use std::{collections::HashMap, path::Path};

    use super::{assert_eq, *};
    use foundry_compilers::artifacts::SolcInput;
    use solidity_ast::{ProjectConfigInputBuilder, Result};

    pub fn get_compiler_input(root: &str) -> Result<HashMap<semver::Version, SolcInput>> {
        let config_input = ProjectConfigInputBuilder::new(Path::new(root)).build()?;
        let ast_input = config_input.solc_input_for_ast_generation()?;
        Ok(ast_input)
    }
}

#[allow(unused_imports)]
mod foundry_basic {
    use foundry_compilers::solc::Solc;

    use crate::common::get_compiler_input;

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-basic";

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.values().len(), 1); // 1 version group
    }
}

#[allow(unused_imports)]
mod foundry_soldeer_basic {

    use foundry_compilers::solc::Solc;

    use super::{assert_eq, *};

    use crate::common::get_compiler_input;

    const ROOT: &str = "test-configs/foundry-soldeer-basic";

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.values().len(), 1); // 1 version group
    }
}

#[allow(unused_imports)]
mod soldeer_basic {
    use super::{assert_eq, *};

    use crate::common::get_compiler_input;

    const ROOT: &str = "test-configs/soldeer-basic";

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.values().len(), 1); // 1 version group
    }
}

#[allow(unused_imports)]
mod foundry_soldeer_dep {
    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-soldeer-dep";

    use crate::common::get_compiler_input;

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.values().len(), 1); // 1 version group
        let values = c.values().next().expect("No files found");
        assert_eq!(values.sources.len(), 1);
    }
}

#[allow(unused_imports)]
mod foundry_soldeer_dep_noremap {

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-soldeer-dep-noremap";
    use crate::common::get_compiler_input;

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.values().len(), 1); // 1 version group
        let values = c.values().next().expect("No files found");
        assert_eq!(values.sources.len(), 1);
    }
}

#[allow(unused_imports)]
mod hardhat_basic {

    use super::{assert_eq, *};
    use crate::common::get_compiler_input;

    const ROOT: &str = "test-configs/hardhat-basic";

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.values().len(), 1); // 1 version group
        let values = c.values().next().expect("No files found");
        assert_eq!(values.sources.len(), 2);
    }
}

#[allow(unused_imports)]
mod hardhat_mini {

    use std::path::Path;

    use itertools::Itertools;
    use solidity_ast::ProjectConfigInputBuilder;

    use super::{assert_eq, *};
    use crate::common::get_compiler_input;

    const ROOT: &str = "test-configs/hardhat-mini";

    // NOTE: Uses custom project config

    #[test]
    fn compiler_input() {
        let builder = ProjectConfigInputBuilder::new(Path::new(ROOT))
            .with_exclude(solidity_ast::ExcludeConfig::Specific(vec![".t.sol".to_string()]));
        let config_input = builder.build().unwrap();
        let c = config_input.solc_input_for_ast_generation().unwrap();
        assert_eq!(c.values().len(), 1); // 1 version group
        let values = c.values().next().expect("No files found");
        let source_files = values.sources.keys().collect_vec();
        assert!(source_files[0].ends_with("contracts/Counter.sol"));
        assert!(source_files[1].ends_with("contracts/ReverseCounter.sol"));
        assert!(source_files[2].ends_with("src/console.sol"));
        assert!(source_files[3].ends_with("src/console2.sol"));
        assert_eq!(source_files.len(), 4);
    }
}

#[allow(unused_imports)]
mod foundry_fix_version {

    use semver::Version;

    use crate::common::get_compiler_input;

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-fix-version";

    #[test]
    fn compiler_input() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.len(), 1); // 1 version group
        let version = c.keys().next().expect("no version group found");
        assert_eq!(version.major, 0); // as declared in foundry.toml
        assert_eq!(version.minor, 8);
        assert_eq!(version.patch, 15);
        let values = c.values().next().expect("No files found");
        assert_eq!(values.sources.len(), 1);
    }
}

#[allow(unused_imports)]
mod foundry_evm_version {
    use foundry_compilers::artifacts::EvmVersion;

    use crate::common::get_compiler_input;

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-evm-version";

    #[test]
    fn compiler_input_respects_configured_evm_version() {
        let c = get_compiler_input(ROOT).unwrap();
        assert_eq!(c.len(), 1);

        let values = c.values().next().expect("No files found");
        assert_eq!(values.settings.evm_version, Some(EvmVersion::Osaka));
    }
}
