use dotos::{DotosEncode, DotosSource};
use meta_signal_mirror::{z2VUH6, z2VWt5};

#[test]
fn canonical_dotos_examples_are_exact_root_witnesses() {
    let examples = include_str!("../examples/canonical.dotos");
    let values = examples
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with(";;"))
        .collect::<Vec<_>>();

    assert_eq!(values.len(), 11);
    for text in &values[..5] {
        let value = DotosSource::new(text)
            .parse::<z2VWt5>()
            .expect("canonical request decodes");
        assert_eq!(value.to_dotos(), *text);
    }
    for text in &values[5..] {
        let value = DotosSource::new(text)
            .parse::<z2VUH6>()
            .expect("canonical reply decodes");
        assert_eq!(value.to_dotos(), *text);
    }
}
