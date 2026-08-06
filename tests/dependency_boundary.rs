use std::{fs, path::PathBuf, process::Command};

fn cargo_tree(edges: &str, extra: &[&str]) -> String {
    let mut command = Command::new("cargo");
    command.args(["tree", "--edges", edges, "--no-default-features"]);
    command.args(extra);
    let output = command.output().expect("run cargo tree");
    assert!(output.status.success(), "status: {:?}", output.status);
    String::from_utf8(output.stdout).expect("dependency tree")
}

#[test]
fn default_runtime_tree_excludes_bootstrap_and_retired_crates() {
    let tree = cargo_tree("normal", &[]);
    assert!(tree.contains("signal-mirror v0.2.0"), "{tree}");
    assert!(tree.contains("signal-standard v0.3.0"), "{tree}");
    for forbidden in [
        "core-ethos",
        "name-table",
        "nota",
        "rust-logos",
        concat!("schema", "-language"),
        "schema-rust",
        "sema-translator",
        "signal-sema-translator",
        "structural-codec",
    ] {
        assert!(
            !tree.contains(forbidden),
            "runtime contains {forbidden}:\n{tree}"
        );
    }
}

#[test]
fn build_tree_has_exact_current_generator_and_exact_producers() {
    let tree = cargo_tree("build", &[]);
    let generator_lines = tree
        .lines()
        .filter(|line| {
            line.starts_with("├── schema-rust v") || line.starts_with("└── schema-rust v")
        })
        .collect::<Vec<_>>();
    assert_eq!(generator_lines.len(), 1, "{tree}");
    assert!(
        generator_lines.iter().all(|line| line.contains(
            "schema-rust v0.15.1 (https://github.com/LiGoldragon/schema-rust.git?rev=664335240a40728826cfaa09e3100cd867031912#66433524)"
        )),
        "{tree}"
    );
    assert!(
        tree.contains("signal-mirror.git?rev=d57830076840f0e1ef89352823d7b0356a7c96df#d5783007")
    );
    assert!(
        tree.contains("signal-standard.git?rev=d5a4a545e61dafec30667f2af38ca503ab6d6d3f#d5a4a545")
    );
    assert!(!tree.contains(concat!("schema", "-language")), "{tree}");

    let lock = include_str!("../Cargo.lock");
    assert!(lock.contains(
        "git+https://github.com/LiGoldragon/schema-rust.git?rev=664335240a40728826cfaa09e3100cd867031912#664335240a40728826cfaa09e3100cd867031912"
    ));
}

#[test]
fn dotos_is_the_only_text_projection_opt_in() {
    let tree = cargo_tree("normal", &["--features", "dotos-text"]);
    assert!(tree.contains("dotos"), "{tree}");
    assert!(!tree.contains("nota"), "{tree}");
}

#[test]
fn obsolete_emitter_vocabulary_is_absent_from_active_build_inputs() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let build = fs::read_to_string(root.join("build.rs")).expect("build source");
    let cargo = fs::read_to_string(root.join("Cargo.toml")).expect("cargo manifest");
    for obsolete in [
        concat!("Generation", "Driver"),
        concat!("Generation", "Plan"),
        concat!("Module", "Emission"),
        concat!("ContractCrate", "Build"),
        concat!("CargoSchema", "Metadata"),
        concat!("Dependency", "Schema"),
        concat!("schema", "-language"),
    ] {
        assert!(!build.contains(obsolete), "build.rs contains {obsolete}");
        assert!(!cargo.contains(obsolete), "Cargo.toml contains {obsolete}");
    }
}
