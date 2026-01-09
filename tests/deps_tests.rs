//! Test the dependencies and challenge assumptions.
//! Allows us to know how we can use these in library

use foundry_compilers::solc::SolcCompiler;
use foundry_rs_config::Config;
use pretty_assertions::assert_eq;
use std::{ffi::OsStr, path::Path};

mod common {
    use std::collections::HashMap;

    use foundry_compilers::{
        Graph, ProjectBuilder,
        artifacts::{Settings, SolcInput, Sources, output_selection::OutputSelection},
        resolver::parse::SolParser,
        solc::{Solc, SolcCompiler, SolcLanguage},
    };
    use semver::Version;

    use super::{assert_eq, *};

    pub fn get_raw_config(root: &str) -> Config {
        assert!(Path::new(root).exists());
        Config::load_with_root(root).unwrap() /*.sanitized()*/
    }

    pub fn can_be_grouped(root: &str) {
        let g = get_group(root);
        assert!(!g.is_empty());
    }

    pub fn get_group(root: &str) -> HashMap<semver::Version, Sources> {
        let raw_config = common::get_raw_config(root).sanitized();
        let project_paths = raw_config.project_paths();

        let graph = Graph::<SolParser>::resolve_sources(
            &project_paths,
            project_paths.read_sources().unwrap(),
        )
        .unwrap();

        let p = ProjectBuilder::<SolcCompiler>::default()
            .paths(project_paths.clone())
            .build(Default::default())
            .unwrap();

        let resolved = graph.into_sources_by_version(&p).unwrap();
        let versioned_sources = resolved.sources.get(&SolcLanguage::Solidity).unwrap().clone();
        versioned_sources.into_iter().map(|(v, s, _)| (v, s)).collect::<HashMap<_, _>>()
    }

    #[test]
    fn solc_can_be_installed() {
        // Make sure to check [`Solc::releases`] to see what's available first.
        // If that is not done, it will install but erorr out. (As it may be Pre-relase)
        let solc = Solc::find_or_install(&Version::new(0, 8, 28)).unwrap();
        assert!(solc.solc.exists());
        let solc = Solc::find_or_install(&Version::new(0, 8, 29)).unwrap();
        assert!(solc.solc.exists());
    }

    #[test]
    fn can_create_stdin_json() {
        const ROOT: &str = "test-configs/foundry-basic";
        let raw_config = common::get_raw_config(ROOT).sanitized();
        let project_paths = raw_config.project_paths::<SolcLanguage>();

        // TODO: set remapppings in settings
        let settings = Settings::new(OutputSelection::ast_output_selection());
        let solc_input =
            SolcInput::new(SolcLanguage::Solidity, project_paths.read_sources().unwrap(), settings);
        let stdin_json = serde_json::to_string(&solc_input).unwrap();
        assert!(!stdin_json.is_empty());
    }

    #[test]
    fn can_handle_symlinks() {
        const ROOT: &str = "test-configs/foundry-symlink";
        let raw_config = common::get_raw_config(ROOT).sanitized();
        let project_paths = raw_config.project_paths::<SolcLanguage>();
        assert_eq!(project_paths.read_sources().unwrap().len(), 2);
    }

    #[test]
    fn can_read_symlink() {
        let path = Path::new("test-configs/foundry-symlink/src/TimerLink.sol");
        let content = std::fs::read_to_string(path);
        assert!(path.exists());
        assert!(content.is_ok());
    }

    // NOTE:
    // * [`Config::project_paths::<Solc>()`] has a bug which is that the `sources` field
    // on [`ProjectPaths`] although correctly identified in `Config`, get canonicalized
    // incorrectly when the value is `src`. Otherwise, canonicalization won't happen (as desired)
    // Above observation has been made on non-sanitized config.
    // DO NOT TRUST .sources if .sanitized() IS NOT CALLED
    // TRUST ONLY IF .sanitized() IS CALLED ON `Config`
    // Update: Maybe this bug is only while displaying. Testing seems to work
}

/// Module created with plain `forge init` and nothing more .
mod foundry_basic {
    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-basic";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.remappings.len(), 1);
    }

    #[test]
    fn identifies_source_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }

    #[test]
    fn identified_version_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        let compiler = raw_config.solc_compiler().unwrap();
        assert!(matches!(compiler, SolcCompiler::AutoDetect));
    }
}

mod foundry_soldeer_basic {
    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-soldeer-basic";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.remappings.len(), 2);
    }

    #[test]
    fn identifies_source_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }

    #[test]
    fn identified_version_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        let compiler = raw_config.solc_compiler().unwrap();
        assert!(matches!(compiler, SolcCompiler::AutoDetect));
    }
}

mod soldeer_basic {
    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/soldeer-basic";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.remappings.len(), 1); // reads from remappings.txt
    }

    #[test]
    //WARN: INCORRECT IDENTIFICATION OF SOURCE
    fn identifies_source_incorrectly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }

    #[test]
    fn identified_version_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        let compiler = raw_config.solc_compiler().unwrap();
        assert!(matches!(compiler, SolcCompiler::AutoDetect));
    }
}

mod foundry_soldeer_dep {
    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-soldeer-dep";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.remappings.len(), 2);
    }

    #[test]
    fn identifies_source_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }

    #[test]
    fn identified_version_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        let compiler = raw_config.solc_compiler().unwrap();
        assert!(matches!(compiler, SolcCompiler::AutoDetect));
    }
}

mod foundry_soldeer_dep_noremap {

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-soldeer-dep-noremap";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.remappings.len(), 6);
    }

    #[test]
    fn identifies_source_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }

    #[test]
    fn identified_version_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        let compiler = raw_config.solc_compiler().unwrap();
        assert!(matches!(compiler, SolcCompiler::AutoDetect));
    }
}

#[allow(unused_imports)]
mod hardhat_basic {

    use foundry_compilers::{
        Graph, Project, ProjectBuilder, ProjectCompileOutput,
        project::{self, ProjectCompiler},
        resolver::parse::SolData,
        solc::{Solc, SolcCompiler, SolcLanguage},
    };

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/hardhat-basic";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        let remaps = raw_config.get_all_remappings().collect::<Vec<_>>();
        assert_eq!(remaps[0].name, "hardhat/");
        assert_eq!(raw_config.remappings.len(), 1);
    }

    #[test]
    fn identifies_source_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("contracts"));
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }

    #[test]
    fn group_contains_right_sources() {
        let group = common::get_group(ROOT);
        let sources = group.values().next().expect("no sources prsent");
        assert_eq!(
            sources
                .keys()
                .filter(|k| k.ends_with("console.sol") || k.ends_with("Lock.sol"))
                .collect::<Vec<_>>()
                .len(),
            2
        );
        assert_eq!(sources.len(), 2);
    }

    #[test]
    fn identified_version_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        let compiler = raw_config.solc_compiler().unwrap();
        // NOTE: This is incorrect. It's actually a fixed version as declared in hardhat.config.ts
        assert!(matches!(compiler, SolcCompiler::AutoDetect));
    }
}

#[allow(unused_imports)]
mod hardhat_dupsource {

    use foundry_compilers::{
        Graph, Project, ProjectBuilder, ProjectCompileOutput,
        project::{self, ProjectCompiler},
        resolver::parse::SolData,
        solc::{Solc, SolcCompiler, SolcLanguage},
    };
    use itertools::Itertools;

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/hardhat-dupsource";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        let remaps = raw_config.get_all_remappings().collect::<Vec<_>>();
        assert_eq!(remaps[0].name, "eth-gas-reporter/");
        assert_eq!(remaps[1].name, "hardhat/");
        assert_eq!(raw_config.remappings.len(), 2);
    }

    #[test]
    fn identifies_source_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("contracts"));
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }

    #[test]
    fn group_contains_right_sources() {
        let group = common::get_group(ROOT);
        let sources = group.values().next().expect("no sources prsent");
        assert_eq!(sources.keys().collect::<Vec<_>>().len(), 29); // forge-std is builtin contracts/
        // In ast_tests.rs, we only see 3 sources
    }

    #[test]
    fn identified_version_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        let compiler = raw_config.solc_compiler().unwrap();
        // NOTE: This is incorrect. It's actually a fixed version as declared in hardhat.config.ts
        assert!(matches!(compiler, SolcCompiler::AutoDetect));
    }
}

#[allow(unused_imports)]
mod foundry_fix_version {

    use foundry_compilers::{
        Graph, Project, ProjectBuilder, ProjectCompileOutput,
        project::{self, ProjectCompiler},
        resolver::parse::SolData,
        solc::{Solc, SolcCompiler, SolcLanguage},
    };
    use semver::Version;

    use super::{assert_eq, *};

    const ROOT: &str = "test-configs/foundry-fix-version";

    #[test]
    fn identifies_remappings_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.remappings.len(), 1);
    }

    #[test]
    fn identifies_source_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        assert_eq!(raw_config.src.as_os_str(), OsStr::new("src"));
    }

    #[test]
    fn identified_version_correctly() {
        let raw_config = common::get_raw_config(ROOT);
        let compiler = raw_config.solc_compiler().unwrap();
        match compiler {
            SolcCompiler::AutoDetect => panic!("explcit version did not override auto detect solc"),
            SolcCompiler::Specific(solc) => {
                assert_eq!(solc.version.major, 0);
                assert_eq!(solc.version.minor, 8);
                assert_eq!(solc.version.patch, 15);
            }
        }
    }

    #[test]
    fn can_be_grouped() {
        common::can_be_grouped(ROOT);
    }
}
