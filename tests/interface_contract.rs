#[test]
fn current_schema_preserves_all_operations_and_configuration_payload() {
    let source = meta_signal_mirror::META_MIRROR_SIGNAL_SOURCE;
    for form in [
        "RetireStore.StoreRetirement",
        "RegisterStore.StoreRegistration",
        "Configure.ConfigureRequest",
        "ObserveRegistry.RegistryQuery",
        "SetRetention.RetentionOrder",
        "DaemonConfiguration.{ StoragePath WorkingSocketBinding MetaSocketBinding NetworkEndpoint }",
    ] {
        assert!(source.contains(form), "{form}");
    }
    assert!(meta_signal_mirror::META_MIRROR_SIGNAL_RUST.contains("pub enum Query"));
    assert!(meta_signal_mirror::META_MIRROR_SIGNAL_RUST.contains("pub enum Response"));
}
