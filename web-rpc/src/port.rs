use std::rc::Rc;
use wasm_bindgen::JsValue;

/// Port abstracts over the different Javascript types that support sending and
/// receiving messages. We need to abstract over these types since there is no
/// trait available to describe this interface. Moreover, some of these interfaces
/// have slightly different semantics, e.g., we need to call [web_sys::MessagePort::start]
/// for the [web_sys::MessagePort]. The inner variants are wrapped in an [Rc] so that
/// we can force a worker to terminate when we drop the last instance. Unfortunately,
/// the browser does not seem to reliably terminate workers during garbage collection.
#[derive(Clone)]
pub enum Port {
    Worker(Rc<web_sys::Worker>),
    DedicatedWorkerGlobalScope(Rc<web_sys::DedicatedWorkerGlobalScope>),
    MessagePort(Rc<web_sys::MessagePort>),
}

impl Port {
    /// Dispatch `post_message` for the different implementations
    pub fn post_message(&self, message: &JsValue, transfer: &JsValue) -> Result<(), JsValue> {
        match self {
            Port::Worker(worker) => worker.post_message_with_transfer(message, transfer),
            Port::DedicatedWorkerGlobalScope(scope) => {
                scope.post_message_with_transfer(message, transfer)
            }
            Port::MessagePort(port) => port.post_message_with_transferable(message, transfer),
        }
    }

    pub(crate) fn start(&self) {
        if let Port::MessagePort(port) = self {
            port.start()
        }
    }

    pub(crate) fn event_target(&self) -> &web_sys::EventTarget {
        match self {
            Port::Worker(worker) => worker.as_ref(),
            Port::DedicatedWorkerGlobalScope(scope) => scope.as_ref(),
            Port::MessagePort(port) => port.as_ref(),
        }
    }
}

impl From<web_sys::Worker> for Port {
    fn from(worker: web_sys::Worker) -> Self {
        Port::Worker(worker.into())
    }
}

impl From<web_sys::DedicatedWorkerGlobalScope> for Port {
    fn from(scope: web_sys::DedicatedWorkerGlobalScope) -> Self {
        Port::DedicatedWorkerGlobalScope(scope.into())
    }
}

impl From<web_sys::MessagePort> for Port {
    fn from(port: web_sys::MessagePort) -> Self {
        Port::MessagePort(port.into())
    }
}

impl Drop for Port {
    fn drop(&mut self) {
        if let Port::Worker(worker) = self {
            if Rc::strong_count(worker) == 1 {
                worker.terminate()
            }
        }
    }
}
