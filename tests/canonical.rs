//! `examples/canonical.datom` is the authored wire-text witness of this
//! contract. Every non-comment line is one encoded root. The test actualizes
//! each line through the codec, re-textualizes the value, and requires the
//! rendering to reproduce the authored line byte for byte, so the file cannot
//! drift from what the codec actually reads and writes.
#![cfg(feature = "datom")]

use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use meta_signal_mirror::{Query, Response};
use protos::{Protosizable, ReaderBudget, Textualizable};

const CANONICAL: &str = include_str!("../examples/canonical.datom");

fn budget() -> Budget {
    Budget {
        remaining: 1 << 20,
        reader: ReaderBudget { remaining: 1 << 20 },
        depth: 0,
        maximum_depth: 256,
    }
}

fn lines() -> Vec<&'static str> {
    CANONICAL
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with(';'))
        .collect()
}

fn query_tag(query: &Query) -> &'static str {
    match query {
        Query::RetireStore(_) => "RetireStore",
        Query::RegisterStore(_) => "RegisterStore",
        Query::Configure(_) => "Configure",
        Query::ObserveRegistry(_) => "ObserveRegistry",
        Query::SetRetention(_) => "SetRetention",
    }
}

fn response_tag(response: &Response) -> &'static str {
    match response {
        Response::StoreRegistered(_) => "StoreRegistered",
        Response::RegistryObserved(_) => "RegistryObserved",
        Response::RetentionSet(_) => "RetentionSet",
        Response::OrderRejected(_) => "OrderRejected",
        Response::StoreRetired(_) => "StoreRetired",
        Response::Configured(_) => "Configured",
    }
}

/// The set of roots the canonical file must exercise. `query_tag` and
/// `response_tag` match exhaustively, so a root added to the Ethos source
/// fails to compile here until it is named and given a canonical line.
const QUERY_ROOTS: [&str; 5] = [
    "RetireStore",
    "RegisterStore",
    "Configure",
    "ObserveRegistry",
    "SetRetention",
];
const RESPONSE_ROOTS: [&str; 6] = [
    "StoreRegistered",
    "RegistryObserved",
    "RetentionSet",
    "OrderRejected",
    "StoreRetired",
    "Configured",
];

macro_rules! render {
    ($value:expr) => {
        $value.clone().datomize(vec![]).protosize().textualize()
    };
}

/// Each authored line actualizes as exactly one root type, and rendering the
/// value back reproduces the line. Both directions of the codec run here.
#[test]
fn every_canonical_line_actualizes_and_renders_back_to_itself() {
    let mut queries = Vec::new();
    let mut responses = Vec::new();

    for line in lines() {
        let as_query = Potential::<Query>::from(String::from(line)).actualize(&mut budget());
        let as_response = Potential::<Response>::from(String::from(line)).actualize(&mut budget());

        match (as_query, as_response) {
            (Ok(query), Err(_)) => {
                assert_eq!(render!(query), line, "query line does not render back");
                queries.push(query_tag(&query));
            }
            (Err(_), Ok(response)) => {
                assert_eq!(
                    render!(response),
                    line,
                    "response line does not render back"
                );
                responses.push(response_tag(&response));
            }
            (Ok(_), Ok(_)) => panic!("line reads as both a Query and a Response: {line}"),
            (Err(query_error), Err(response_error)) => panic!(
                "line reads as neither root: {line}\n  as Query: {query_error:?}\n  as Response: {response_error:?}"
            ),
        }
    }

    queries.sort_unstable();
    responses.sort_unstable();
    let mut expected_queries = QUERY_ROOTS;
    let mut expected_responses = RESPONSE_ROOTS;
    expected_queries.sort_unstable();
    expected_responses.sort_unstable();

    assert_eq!(queries, expected_queries, "request roots not fully covered");
    assert_eq!(
        responses, expected_responses,
        "reply roots not fully covered"
    );
}
