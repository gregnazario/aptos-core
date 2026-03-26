// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License, available at https://github.com/aptos-labs/aptos-core/blob/main/LICENSE

use crate::{
    common::PACKAGE_NAME_DELIMITER,
    subsystems::{affected_subsystems, has_safety_critical_changes},
};
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

/// Configuration for a single Forge test suite, loaded from TOML config.
#[derive(Debug, Deserialize)]
struct ForgeSuiteConfig {
    subsystems: Vec<String>,
    #[serde(default = "default_safety")]
    safety: String,
}

fn default_safety() -> String {
    "normal".to_string()
}

/// Top-level structure of the forge-suite-subsystems.toml config file.
#[derive(Debug, Deserialize)]
struct ForgeSuitesConfig {
    #[serde(default)]
    suites: BTreeMap<String, ForgeSuiteConfig>,
}

/// The JSON test plan output consumed by GitHub Actions.
#[derive(Debug, Serialize)]
pub struct TestPlan {
    /// Which subsystems were detected as affected by the changes.
    pub subsystems_affected: BTreeSet<String>,

    /// Affected crate package names for targeted unit tests.
    pub unit_test_packages: Vec<String>,

    /// Whether smoke tests should run.
    pub run_smoke_tests: bool,

    /// Whether Forge e2e tests should run.
    pub run_forge_e2e: bool,

    /// Whether Forge compat tests should run.
    pub run_forge_compat: bool,

    /// Whether framework upgrade tests should run.
    pub run_framework_upgrade: bool,

    /// Whether execution performance tests should run.
    pub run_execution_performance: bool,

    /// List of Forge suite names that should be skipped.
    pub forge_suites_to_skip: Vec<String>,

    /// If true, safety-critical code changed -- all tests must run.
    pub safety_override: bool,
}

impl TestPlan {
    /// Generate a test plan from changed files and affected packages.
    ///
    /// `changed_files`: list of file paths changed relative to merge-base.
    /// `affected_packages`: full package paths from the determinator (with `#name` suffix).
    /// `forge_config_path`: optional path to the forge-suite-subsystems.toml file.
    pub fn generate(
        changed_files: &[String],
        affected_packages: &[String],
        forge_config_path: Option<&Path>,
    ) -> Self {
        // Determine affected subsystems from changed files
        let subsystems_affected = affected_subsystems(changed_files.iter().map(|s| s.as_str()));
        let safety_override = has_safety_critical_changes(&subsystems_affected);

        if safety_override {
            info!("Safety-critical subsystem changed -- forcing all tests to run");
        }

        // Extract package names from full paths
        let package_names: Vec<String> = affected_packages
            .iter()
            .filter_map(|p| p.split(PACKAGE_NAME_DELIMITER).last())
            .filter(|name| !name.is_empty())
            .map(|s| s.to_string())
            .collect();

        // Determine which high-level test categories to run based on subsystems
        let run_smoke_tests = safety_override
            || subsystems_affected
                .iter()
                .any(|s| matches!(s.as_str(), "consensus" | "execution" | "storage" | "network" | "mempool" | "state-sync" | "move-vm" | "move-framework"));

        let run_forge_e2e = run_smoke_tests; // Same subsystem triggers

        let run_forge_compat = safety_override
            || subsystems_affected
                .iter()
                .any(|s| matches!(s.as_str(), "consensus" | "network" | "state-sync"));

        let run_framework_upgrade = safety_override
            || subsystems_affected
                .iter()
                .any(|s| matches!(s.as_str(), "move-framework" | "execution"));

        let run_execution_performance = safety_override
            || subsystems_affected
                .iter()
                .any(|s| matches!(s.as_str(), "execution" | "move-vm" | "storage"));

        // Determine Forge suites to skip from config file
        let forge_suites_to_skip = if safety_override {
            vec![] // Don't skip anything when safety-critical
        } else {
            compute_forge_skips(&subsystems_affected, forge_config_path)
        };

        TestPlan {
            subsystems_affected,
            unit_test_packages: package_names,
            run_smoke_tests,
            run_forge_e2e,
            run_forge_compat,
            run_framework_upgrade,
            run_execution_performance,
            forge_suites_to_skip,
            safety_override,
        }
    }

    /// Serialize the test plan to JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("failed to serialize test plan")
    }
}

/// Load the Forge suite config and determine which suites can be skipped
/// because none of their required subsystems are in the affected set.
fn compute_forge_skips(
    affected: &BTreeSet<String>,
    config_path: Option<&Path>,
) -> Vec<String> {
    let config_path = match config_path {
        Some(p) if p.exists() => p,
        _ => return vec![],
    };

    let contents = match fs::read_to_string(config_path) {
        Ok(c) => c,
        Err(e) => {
            info!("Could not read forge suite config: {}", e);
            return vec![];
        },
    };

    let config: ForgeSuitesConfig = match toml::from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            info!("Could not parse forge suite config: {}", e);
            return vec![];
        },
    };

    let mut skips = Vec::new();
    for (suite_name, suite_config) in &config.suites {
        // Never skip critical suites unless we're certain
        if suite_config.safety == "critical" {
            continue;
        }

        // Check if any of the suite's required subsystems are affected
        let suite_affected = suite_config
            .subsystems
            .iter()
            .any(|s| affected.contains(s));

        if !suite_affected {
            skips.push(suite_name.clone());
        }
    }

    skips
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_empty_changes() {
        let plan = TestPlan::generate(&[], &[], None);
        assert!(plan.subsystems_affected.is_empty());
        assert!(!plan.run_smoke_tests);
        assert!(!plan.run_forge_e2e);
        assert!(!plan.safety_override);
    }

    #[test]
    fn test_generate_consensus_change() {
        let changed = vec!["consensus/src/lib.rs".to_string()];
        let packages = vec!["file:///aptos-core/consensus#aptos-consensus".to_string()];
        let plan = TestPlan::generate(&changed, &packages, None);

        assert!(plan.subsystems_affected.contains("consensus"));
        assert!(plan.run_smoke_tests);
        assert!(plan.run_forge_e2e);
        assert!(plan.run_forge_compat);
        assert!(!plan.run_framework_upgrade);
        assert!(!plan.safety_override);
        assert_eq!(plan.unit_test_packages, vec!["aptos-consensus"]);
    }

    #[test]
    fn test_generate_crypto_forces_all() {
        let changed = vec!["crates/aptos-crypto/src/lib.rs".to_string()];
        let plan = TestPlan::generate(&changed, &[], None);

        assert!(plan.safety_override);
        assert!(plan.run_smoke_tests);
        assert!(plan.run_forge_e2e);
        assert!(plan.run_forge_compat);
        assert!(plan.run_framework_upgrade);
        assert!(plan.run_execution_performance);
        assert!(plan.forge_suites_to_skip.is_empty());
    }

    #[test]
    fn test_forge_skip_from_config() {
        let affected: BTreeSet<String> = ["consensus".to_string()].into();
        let config_toml = r#"
[suites.framework_upgrade]
subsystems = ["move-framework", "execution"]
safety = "normal"

[suites.compat]
subsystems = ["consensus", "network"]
safety = "critical"

[suites.perf_test]
subsystems = ["execution"]
safety = "normal"
"#;
        // Write temp config
        let dir = tempfile::tempdir().unwrap();
        let config_path = dir.path().join("forge-suite-subsystems.toml");
        fs::write(&config_path, config_toml).unwrap();

        let skips = compute_forge_skips(&affected, Some(&config_path));
        // framework_upgrade should be skipped (not affected), perf_test should be skipped
        // compat should NOT be skipped (critical safety)
        assert!(skips.contains(&"framework_upgrade".to_string()));
        assert!(skips.contains(&"perf_test".to_string()));
        assert!(!skips.contains(&"compat".to_string()));
    }

    #[test]
    fn test_json_output() {
        let plan = TestPlan::generate(&[], &[], None);
        let json = plan.to_json();
        assert!(json.contains("subsystems_affected"));
        assert!(json.contains("safety_override"));
    }
}
