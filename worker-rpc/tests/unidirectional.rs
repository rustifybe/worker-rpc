use futures_util::FutureExt;
use wasm_bindgen_test::*;

#[worker_rpc::service]
pub trait Service {
    fn add(left: u32, right: u32) -> u32;
}
struct ServiceServerImpl;
impl Service for ServiceServerImpl {
    fn add(&self, left: u32, right: u32) -> u32 {
        left + right
    }
}

#[wasm_bindgen_test]
async fn unidirectional() {
    console_error_panic_hook::set_once();
    /* create channel */
    let channel = web_sys::MessageChannel::new().unwrap();
    /* create and spawn server (shuts down when _server_handle is dropped) */
    let (server, _server_handle) = worker_rpc::Builder::new(channel.port1())
        .with_server(ServiceServer::new(ServiceServerImpl))
        .build().await
        .remote_handle();
    wasm_bindgen_futures::spawn_local(server);
    /* create client */
    let client = worker_rpc::Builder::new(channel.port2())
        .with_client::<ServiceClient>()
        .build().await;
    /* run test */
    assert_eq!(client.add(41, 1).await.expect("RPC failure"), 42);
}

