//! Smoke tests for the patterns a production CDP client relies on:
//! generic command encoding, typed decoding, typed events, and error replies.

use browser_protocol::page::{FrameNavigated, LoadEventFired, NavigateParams, TransitionType};
use browser_protocol::{CdpCommand, CdpEvent, CdpReply, Command, EmptyReturns, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Generic request encoder: `CdpCommand` provides the wire method, `Serialize` the body.
fn encode<'a, P>(id: u64, params: &'a P) -> String
where
    P: CdpCommand<'a> + Serialize,
{
    serde_json::to_string(&Command::new(id, params)).unwrap()
}

/// Generic response decoder: correlates by id and yields the typed `Response`.
fn decode<'a, P>(json: &'a str) -> Response<P::Response>
where
    P: CdpCommand<'a>,
    P::Response: DeserializeOwned,
{
    serde_json::from_str(json).unwrap()
}

#[test]
fn command_encodes_to_wire_format() {
    let params = NavigateParams::builder("https://example.com/page")
        .transition_type(TransitionType::Typed)
        .build();

    assert_eq!(
        encode(7, &params),
        r#"{"id":7,"method":"Page.navigate","params":{"url":"https://example.com/page","transitionType":"typed"}}"#
    );
}

#[test]
fn response_decodes_to_typed_result() {
    let response = decode::<NavigateParams>(r#"{"id":7,"result":{"frameId":"FRAME-1"}}"#);

    assert_eq!(response.id, 7);
    assert_eq!(response.result.frame_id(), "FRAME-1");
    assert_eq!(response.result.loader_id(), None);
}

#[test]
fn empty_returns_round_trips() {
    use browser_protocol::browser::GetVersionParams;
    let params = GetVersionParams::default();
    assert_eq!(
        encode(1, &params),
        r#"{"id":1,"method":"Browser.getVersion","params":{}}"#
    );
}

#[test]
fn events_are_typed_and_carry_method() {
    assert_eq!(LoadEventFired::METHOD, "Page.loadEventFired");
    assert_eq!(FrameNavigated::METHOD, "Page.frameNavigated");

    // The `CdpEvent` trait makes events usable generically.
    fn method_of<E: CdpEvent>() -> &'static str {
        E::METHOD
    }
    assert_eq!(method_of::<LoadEventFired>(), "Page.loadEventFired");

    let event: LoadEventFired = serde_json::from_str(r#"{"timestamp":12.5}"#).unwrap();
    assert_eq!(*event.timestamp(), 12.5);
}

#[test]
fn reply_decodes_success_and_error() {
    let ok: CdpReply<EmptyReturns> = serde_json::from_str(r#"{"id":1,"result":{}}"#).unwrap();
    match ok {
        CdpReply::Ok(response) => assert_eq!(response.id, 1),
        CdpReply::Err(_) => panic!("expected Ok"),
    }

    let err: CdpReply<EmptyReturns> =
        serde_json::from_str(r#"{"id":2,"error":{"code":-32000,"message":"boom"}}"#).unwrap();
    match err {
        CdpReply::Err(response) => {
            assert_eq!(response.id, 2);
            assert_eq!(response.error.code, -32000);
            assert_eq!(response.error.message, "boom");
        }
        CdpReply::Ok(_) => panic!("expected Err"),
    }
}
