use meta_signal_mirror::{Query, RegistryListing, RegistryQuery, Response};

#[test]
fn current_meta_query_and_response_round_trip_as_fresh_bytes() {
    let query = Query::ObserveRegistry(RegistryQuery {});
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&query).expect("archive");
    assert_eq!(
        rkyv::from_bytes::<Query, rkyv::rancor::Error>(&bytes).expect("restore"),
        query
    );
    let response = Response::RegistryObserved(RegistryListing {
        registered_store_vector: vec![],
    });
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&response).expect("archive");
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&bytes).expect("restore"),
        response
    );
}

#[test]
fn malformed_archive_is_rejected() {
    assert!(rkyv::from_bytes::<Query, rkyv::rancor::Error>(&[0, 1, 2]).is_err());
}

#[cfg(feature = "datom")]
#[test]
fn current_meta_query_round_trips_as_datom() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let value = Query::ObserveRegistry(RegistryQuery {});
    let text = value.datomize(vec![]).protosize().textualize();
    let restored = Potential::<Query>::from(text)
        .actualize(&mut Budget {
            remaining: 1024,
            reader: ReaderBudget { remaining: 1024 },
            depth: 0,
            maximum_depth: 1024,
        })
        .expect("actualize");
    assert_eq!(restored, value);
}
