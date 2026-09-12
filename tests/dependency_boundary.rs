use std::process::Command;
#[test]
fn default_graph_excludes_retired_codecs() {
    let output = Command::new("cargo")
        .args(["tree", "--edges", "normal", "--no-default-features"])
        .output()
        .expect("tree");
    assert!(output.status.success());
    let tree = String::from_utf8(output.stdout).expect("utf8");
    for obsolete in [
        "dotos",
        "signal-frame",
        "schema-rust",
        "core-ethos",
        "signal-standard",
    ] {
        assert!(!tree.contains(obsolete), "{obsolete}: {tree}");
    }
}
