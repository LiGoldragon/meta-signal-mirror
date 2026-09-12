use meta_signal_mirror::{
    ByteViewable, Query, RegistryListing, RegistryQuery, Response, Restorable, Signal, Signalizable,
};

#[test]
fn query_restores_from_fresh_peer_bytes() {
    let query = Query::ObserveRegistry(RegistryQuery {});
    let outgoing = query.signalize().expect("archive query");
    assert!(!outgoing.bytes().is_empty());
    let incoming = Signal::<Query>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore query"), query);
}

#[test]
fn response_restores_from_fresh_peer_bytes() {
    let response = Response::RegistryObserved(RegistryListing {
        registered_store_vector: vec![],
    });
    let outgoing = response.signalize().expect("archive response");
    let incoming = Signal::<Response>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore response"), response);
}

#[test]
fn malformed_peer_bytes_are_rejected() {
    let incoming = Signal::<Query>::from(vec![0, 1, 2]);
    assert!(Restorable::<Query>::restore(&incoming).is_err());
}
