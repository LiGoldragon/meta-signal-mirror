use meta_signal_mirror::*;

fn requests() -> Vec<(z2VWt5, &'static str)> {
    vec![
        (
            z2VWt5::z2VXXR(z2VWZw::new(signal_mirror::schema::lib::z2Ve8p::new(
                "mirror-aab".to_owned(),
            ))),
            "RetireStore",
        ),
        (
            z2VWt5::z2VWC2(z2VWBn {
                field_0: signal_mirror::schema::lib::z2Ve8p::new("mirror-aab".to_owned()),
                field_1: z2VMYP::z2Vf8Y,
            }),
            "RegisterStore",
        ),
        (
            z2VWt5::z2VNSh(z2VR1a::new(z2VXab {
                field_0: z2VPES::new("fixture".to_owned()),
                field_1: z2VM9q::new(z2VRXG {
                    field_0: signal_standard::schema::lib::z2VXNY::new("socket-aab".to_owned()),
                    field_1: z2VQot::new(7),
                }),
                field_2: z2VUYq::new(z2VRXG {
                    field_0: signal_standard::schema::lib::z2VXNY::new("socket-aab".to_owned()),
                    field_1: z2VQot::new(7),
                }),
                field_3: signal_standard::schema::lib::z2VaVE {
                    field_0: signal_standard::schema::lib::z2VLyh::new("host-aab".to_owned()),
                    field_1: signal_standard::schema::lib::z2VQaE::new(7474),
                },
            })),
            "Configure",
        ),
        (z2VWt5::z2VMxr(z2VXxR {}), "ObserveRegistry"),
        (
            z2VWt5::z2Vctc(z2VXLU {
                field_0: z2VTJB::z2VcDy,
                field_1: z2VWXC::z2VXbW(z2VQrL::new(7)),
            }),
            "SetRetention",
        ),
    ]
}

fn replies() -> Vec<(z2VUH6, &'static str)> {
    vec![
        (
            z2VUH6::z2VSig(z2VbpU::new(signal_mirror::schema::lib::z2Ve8p::new(
                "mirror-aab".to_owned(),
            ))),
            "StoreRegistered",
        ),
        (
            z2VUH6::z2VN4H(z2VQWA::new(vec![z2VT8Z::new(
                signal_mirror::schema::lib::z2Ve8p::new("mirror-aab".to_owned()),
            )])),
            "RegistryObserved",
        ),
        (
            z2VUH6::z2Vedh(z2VP3W {
                field_0: z2VTJB::z2VcDy,
                field_1: z2VWXC::z2VXbW(z2VQrL::new(7)),
            }),
            "RetentionSet",
        ),
        (
            z2VUH6::z2VXCa(z2Vbcq {
                field_0: z2VYSE::z2VLwk,
                field_1: z2VbDA::new("fixture".to_owned()),
            }),
            "OrderRejected",
        ),
        (
            z2VUH6::z2VdCD(z2Vcin::new(signal_mirror::schema::lib::z2Ve8p::new(
                "mirror-aab".to_owned(),
            ))),
            "StoreRetired",
        ),
        (
            z2VUH6::z2VQdi(z2VNz1::new(z2VXab {
                field_0: z2VPES::new("fixture".to_owned()),
                field_1: z2VM9q::new(z2VRXG {
                    field_0: signal_standard::schema::lib::z2VXNY::new("socket-aab".to_owned()),
                    field_1: z2VQot::new(7),
                }),
                field_2: z2VUYq::new(z2VRXG {
                    field_0: signal_standard::schema::lib::z2VXNY::new("socket-aab".to_owned()),
                    field_1: z2VQot::new(7),
                }),
                field_3: signal_standard::schema::lib::z2VaVE {
                    field_0: signal_standard::schema::lib::z2VLyh::new("host-aab".to_owned()),
                    field_1: signal_standard::schema::lib::z2VQaE::new(7474),
                },
            })),
            "Configured",
        ),
    ]
}

fn exchange(epoch: u64) -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(epoch),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::first(),
    )
}

#[test]
fn every_request_round_trips_through_the_bound_frame() {
    for (request, _head) in requests() {
        let expected = request.clone();
        let encoded = request
            .encode_request_frame(exchange(61))
            .expect("request frame encodes");
        let (decoded_exchange, decoded) =
            ContractMarker::decode_single_request(&encoded).expect("request frame decodes");
        assert_eq!(decoded_exchange, exchange(61));
        assert_eq!(decoded, expected);
    }
}

#[test]
fn every_reply_has_bound_frame_and_rkyv_behavior() {
    for (reply, _head) in replies() {
        let expected = reply.clone();
        let encoded = reply
            .clone()
            .encode_reply_frame(exchange(63))
            .expect("reply frame encodes");
        ContractMarker::decode_frame(&encoded).expect("reply frame decodes");
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).expect("reply archives");
        let recovered =
            rkyv::from_bytes::<z2VUH6, rkyv::rancor::Error>(&archive).expect("reply recovers");
        assert_eq!(recovered, expected);
    }
}

#[cfg(feature = "dotos-text")]
#[test]
fn every_root_round_trips_through_dotos_with_visible_heads() {
    use dotos::{DotosEncode, DotosSource};
    for (request, head) in requests() {
        let text = request.to_dotos();
        assert!(text.starts_with(&format!("{head}.")), "{text}");
        assert_eq!(
            DotosSource::new(&text)
                .parse::<z2VWt5>()
                .expect("request Dotos decodes"),
            request
        );
    }
    for (reply, head) in replies() {
        let text = reply.to_dotos();
        assert!(text.starts_with(&format!("{head}.")), "{text}");
        assert_eq!(
            DotosSource::new(&text)
                .parse::<z2VUH6>()
                .expect("reply Dotos decodes"),
            reply
        );
    }
}

#[test]
fn daemon_configuration_survives_the_binary_startup_archive() {
    let configuration = match requests()
        .into_iter()
        .find(|(_, head)| *head == "Configure")
        .expect("configure fixture")
        .0
    {
        z2VWt5::z2VNSh(request) => request.into_payload(),
        _ => unreachable!("configure fixture has configure variant"),
    };
    let directory = tempfile::tempdir().expect("temp dir");
    let path = directory.path().join("mirror-configuration.rkyv");
    configuration
        .write_binary_file(&path)
        .expect("write binary configuration");
    let decoded = z2VXab::from_binary_path(&path).expect("decode binary configuration");
    assert_eq!(decoded, configuration);
}

#[test]
fn socket_mode_conversion_is_checked() {
    assert_eq!(
        z2VQot::new(u64::from(u32::MAX)).into_u32().expect("fits"),
        u32::MAX
    );
    assert_eq!(
        z2VQot::new(u64::from(u32::MAX) + 1).into_u32(),
        Err(SocketModeRangeError)
    );
}
