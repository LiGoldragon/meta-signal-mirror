//! Round-trip witnesses for the mirror's meta policy contract: every
//! owner operation crosses the length-prefixed rkyv frame and the NOTA
//! text surface, and the typed daemon configuration survives the binary
//! startup archive the daemon decodes.

use meta_signal_mirror::{
    CheckpointKeepCount, ConfigurationReceipt, ConfigureRequest, ContentAddressing,
    DaemonConfiguration, Frame, FrameBody, Input, ListenAddress, OrderRejection,
    OrderRejectionReason, Output, RegisteredStore, RegistrationReceipt, RegistryListing,
    RegistryQuery, RejectionDetail, RetentionOrder, RetentionReceipt, RetentionRule, RetentionScope,
    RetirementReceipt, SocketMode, StoreName, StoreRegistration, StoreRetirement, WirePath,
};
use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SubReply,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn store(name: &str) -> StoreName {
    StoreName::new(name.to_owned())
}

fn configuration() -> DaemonConfiguration {
    DaemonConfiguration {
        storage_path: WirePath::new("/var/lib/mirror/mirror.sema".to_owned()),
        working_socket_path: WirePath::new("/run/mirror/working.sock".to_owned()),
        working_socket_mode: SocketMode::new(0o660),
        meta_socket_path: WirePath::new("/run/mirror/meta.sock".to_owned()),
        meta_socket_mode: SocketMode::new(0o600),
        tcp_listen_address: ListenAddress::new("100.64.0.7:7474".to_owned()),
    }
}

fn assert_request_round_trips(request: Input) {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request {
            request: decoded_request,
            ..
        } => assert_eq!(decoded_request.payloads().head(), &request),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn assert_reply_round_trips(reply: Output) {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply {
            reply: decoded_reply,
            ..
        } => match decoded_reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => assert_eq!(payload, reply),
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            Reply::Rejected { reason } => panic!("unexpected rejected reply: {reason:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

fn assert_nota_round_trips<Value>(value: &Value)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    let recovered = NotaSource::new(&text).parse::<Value>().expect("decode");
    assert_eq!(&recovered, value);
}

#[test]
fn configure_request_round_trips() {
    let request = Input::Configure(ConfigureRequest::new(configuration()));
    assert_request_round_trips(request.clone());
    assert_nota_round_trips(&request);
}

#[test]
fn store_registration_and_retirement_round_trip() {
    for addressing in [ContentAddressing::Opaque, ContentAddressing::SemaVersionedLog] {
        let request = Input::RegisterStore(StoreRegistration {
            store: store("spirit"),
            addressing,
        });
        assert_request_round_trips(request.clone());
        assert_nota_round_trips(&request);
    }
    let retire = Input::RetireStore(StoreRetirement::new(store("spirit")));
    assert_request_round_trips(retire.clone());
    assert_nota_round_trips(&retire);
}

#[test]
fn retention_order_round_trips_for_every_rule_and_scope() {
    for scope in [
        RetentionScope::Store(store("spirit")),
        RetentionScope::AllStores,
    ] {
        for rule in [
            RetentionRule::KeepEverything,
            RetentionRule::KeepLatestCheckpoints(CheckpointKeepCount::new(3)),
        ] {
            let request = Input::SetRetention(RetentionOrder {
                scope: scope.clone(),
                rule: rule.clone(),
            });
            assert_request_round_trips(request.clone());
            assert_nota_round_trips(&request);
        }
    }
}

#[test]
fn observe_registry_round_trips() {
    let request = Input::ObserveRegistry(RegistryQuery {});
    assert_request_round_trips(request.clone());
    assert_nota_round_trips(&request);
}

#[test]
fn replies_round_trip_with_typed_payloads() {
    let replies = [
        Output::Configured(ConfigurationReceipt::new(configuration())),
        Output::StoreRegistered(RegistrationReceipt::new(store("spirit"))),
        Output::StoreRetired(RetirementReceipt::new(store("spirit"))),
        Output::RetentionSet(RetentionReceipt {
            scope: RetentionScope::AllStores,
            rule: RetentionRule::KeepEverything,
        }),
        Output::RegistryObserved(RegistryListing::new(vec![RegisteredStore::new(store(
            "spirit",
        ))])),
        Output::OrderRejected(OrderRejection {
            reason: OrderRejectionReason::StoreUnknown,
            detail: RejectionDetail::new("no such store".to_owned()),
        }),
    ];
    for reply in replies {
        assert_reply_round_trips(reply.clone());
        assert_nota_round_trips(&reply);
    }
}

#[test]
fn daemon_configuration_survives_the_binary_startup_archive() {
    let directory = tempfile::tempdir().expect("temp dir");
    let path = directory.path().join("mirror-configuration.rkyv");
    let configuration = configuration();
    configuration
        .write_binary_file(&path)
        .expect("write binary configuration");
    let decoded =
        DaemonConfiguration::from_binary_path(&path).expect("decode binary configuration");
    assert_eq!(decoded, configuration);
}
