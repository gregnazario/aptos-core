// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License, available at https://github.com/aptos-labs/aptos-core/blob/main/LICENSE

use std::collections::BTreeSet;

/// A subsystem definition: a name and a set of path prefixes that belong to it.
pub struct Subsystem {
    pub name: &'static str,
    pub path_prefixes: &'static [&'static str],
}

/// All known subsystems in the aptos-core repository.
/// Each subsystem maps to directory prefixes that, when changed, indicate that subsystem is affected.
pub const SUBSYSTEMS: &[Subsystem] = &[
    Subsystem {
        name: "consensus",
        path_prefixes: &["consensus/", "crates/aptos-consensus"],
    },
    Subsystem {
        name: "execution",
        path_prefixes: &[
            "execution/",
            "aptos-move/block-executor/",
            "aptos-move/aptos-vm",
        ],
    },
    Subsystem {
        name: "storage",
        path_prefixes: &["storage/", "crates/aptos-db"],
    },
    Subsystem {
        name: "network",
        path_prefixes: &["network/", "crates/aptos-network"],
    },
    Subsystem {
        name: "mempool",
        path_prefixes: &["mempool/"],
    },
    Subsystem {
        name: "api",
        path_prefixes: &["api/", "crates/aptos-api"],
    },
    Subsystem {
        name: "move-vm",
        path_prefixes: &["third_party/move/"],
    },
    Subsystem {
        name: "move-framework",
        path_prefixes: &["aptos-move/framework/"],
    },
    Subsystem {
        name: "state-sync",
        path_prefixes: &["state-sync/"],
    },
    Subsystem {
        name: "crypto",
        path_prefixes: &["crates/aptos-crypto/"],
    },
    Subsystem {
        name: "keyless",
        path_prefixes: &["keyless/", "crates/aptos-keyless"],
    },
    Subsystem {
        name: "secure",
        path_prefixes: &["secure/", "consensus/safety-rules/"],
    },
    Subsystem {
        name: "indexer",
        path_prefixes: &["ecosystem/indexer", "crates/aptos-indexer"],
    },
    Subsystem {
        name: "cli",
        path_prefixes: &["crates/aptos/"],
    },
    Subsystem {
        name: "ci-infra",
        path_prefixes: &[".github/", "docker/", "terraform/", "scripts/"],
    },
];

/// Subsystems that are safety-critical. When any of these are affected,
/// the test plan sets `safety_override = true` to force all tests to run.
pub const SAFETY_CRITICAL_SUBSYSTEMS: &[&str] = &["crypto", "secure", "keyless"];

/// Given a list of changed file paths, determine which subsystems are affected.
pub fn affected_subsystems<'a, I, S>(changed_files: I) -> BTreeSet<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut affected = BTreeSet::new();
    for file in changed_files {
        let path = file.as_ref();
        for subsystem in SUBSYSTEMS {
            for prefix in subsystem.path_prefixes {
                if path.starts_with(prefix) {
                    affected.insert(subsystem.name.to_string());
                    break;
                }
            }
        }
    }
    affected
}

/// Returns true if any safety-critical subsystem is in the affected set.
pub fn has_safety_critical_changes(affected: &BTreeSet<String>) -> bool {
    SAFETY_CRITICAL_SUBSYSTEMS
        .iter()
        .any(|s| affected.contains(*s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affected_subsystems_consensus() {
        let files = vec!["consensus/src/lib.rs"];
        let affected = affected_subsystems(files);
        assert!(affected.contains("consensus"));
        assert!(!affected.contains("storage"));
    }

    #[test]
    fn test_affected_subsystems_multiple() {
        let files = vec![
            "consensus/src/lib.rs",
            "storage/aptosdb/src/lib.rs",
            "crates/aptos-crypto/src/lib.rs",
        ];
        let affected = affected_subsystems(files);
        assert!(affected.contains("consensus"));
        assert!(affected.contains("storage"));
        assert!(affected.contains("crypto"));
    }

    #[test]
    fn test_safety_critical() {
        let mut affected = BTreeSet::new();
        affected.insert("consensus".to_string());
        assert!(!has_safety_critical_changes(&affected));

        affected.insert("crypto".to_string());
        assert!(has_safety_critical_changes(&affected));
    }

    #[test]
    fn test_no_subsystem_for_docs() {
        let files = vec!["developer-docs-site/docs/intro.md"];
        let affected = affected_subsystems(files);
        assert!(affected.is_empty());
    }

    #[test]
    fn test_safety_rules_maps_to_secure() {
        let files = vec!["consensus/safety-rules/src/lib.rs"];
        let affected = affected_subsystems(files);
        assert!(affected.contains("secure"));
        assert!(affected.contains("consensus"));
    }
}
