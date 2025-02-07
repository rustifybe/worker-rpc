use futures_util::FutureExt;
use wasm_bindgen_test::*;

#[web_rpc::service]
pub trait Concat {
    #[post(left, right, return)]
    fn concat_with_space(left: js_sys::JsString, right: js_sys::JsString) -> js_sys::JsString;
}
struct ConcatServiceImpl;
impl Concat for ConcatServiceImpl {
    fn concat_with_space(
        &self,
        left: js_sys::JsString,
        right: js_sys::JsString,
    ) -> js_sys::JsString {
        js_sys::Array::of2(&left, &right).join(" ")
    }
}

#[wasm_bindgen_test]
async fn post() {
    console_error_panic_hook::set_once();
    /* create channel */
    let channel = web_sys::MessageChannel::new().unwrap();
    let (server_interface, client_interface) = futures_util::future::join(
        web_rpc::Interface::new(channel.port1()),
        web_rpc::Interface::new(channel.port2()),
    )
    .await;
    /* create and spawn server (shuts down when _server_handle is dropped) */
    let (server, _server_handle) = web_rpc::Builder::new(server_interface)
        .with_service::<ConcatService<_>>(ConcatServiceImpl)
        .build()
        .remote_handle();
    wasm_bindgen_futures::spawn_local(server);
    /* create client */
    let client = web_rpc::Builder::new(client_interface)
        .with_client::<ConcatClient>()
        .build();
    /* run test */
    let response = client
        .concat_with_space("hello".into(), "world".into())
        .await;
    assert_eq!(response, "hello world");
}
