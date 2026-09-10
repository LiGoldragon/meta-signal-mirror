use ethos_zero::{Actualizing, File, Generating, Potential};
fn main() {
    println!("cargo:rerun-if-changed=ethos/signal.ethos");
    let root = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest"));
    let source = std::fs::read_to_string(root.join("ethos/signal.ethos")).expect("source");
    let actual = Potential::<File>::from(source)
        .actualize()
        .expect("actualize");
    let generated = actual.generate().expect("generate");
    assert_eq!(
        generated,
        std::fs::read_to_string(root.join("src/generated/signal.rs")).expect("generated")
    );
}
