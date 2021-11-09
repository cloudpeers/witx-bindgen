wai_bindgen_rust::export!("./tests/runtime/handles/imports.wai");

use anyhow::Result;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use wai_bindgen_rust::Handle;

static CLOSED3: AtomicU32 = AtomicU32::new(0);

pub struct Imports;

#[derive(Debug)]
pub struct HostState(u32);

pub struct HostState2;

#[derive(Default, Debug)]
pub struct Markdown2 {
    buf: Arc<Mutex<String>>,
}

pub struct OddName;

impl imports::Markdown2 for Markdown2 {
    fn create() -> Handle<Markdown2> {
        Handle::new(Markdown2::default())
    }

    fn append(&self, buf: String) {
        self.buf.lock().unwrap().push_str(&buf);
    }

    fn render(&self) -> String {
        self.buf.lock().unwrap().replace("red", "green")
    }
}

impl imports::OddName for OddName {
    fn create() -> Handle<Self> {
        Handle::new(Self)
    }
    fn frob_the_odd(&self) {}
}

impl imports::Imports for Imports {
    fn host_state_create() -> Handle<HostState> {
        Handle::new(HostState(100))
    }

    fn host_state_get(state: Handle<HostState>) -> u32 {
        state.0
    }

    fn host_state2_create() -> Handle<HostState2> {
        Handle::new(HostState2)
    }

    fn host_state2_saw_close() -> bool {
        CLOSED3.load(Ordering::SeqCst) == 1
    }

    fn drop_host_state2(_state: HostState2) {
        CLOSED3.store(1, Ordering::SeqCst);
    }

    fn two_host_states(_a: Handle<HostState>, _b: Handle<HostState2>) -> (Handle<HostState>, Handle<HostState2>) {
        (Handle::new(HostState(2)), Handle::new(HostState2))
    }

    fn host_state2_param_record(_a: imports::HostStateParamRecord) {}
    fn host_state2_param_tuple(_a: (Handle<HostState2>,)) {}
    fn host_state2_param_option(_a: Option<Handle<HostState2>>) {}
    fn host_state2_param_result(_a: Result<Handle<HostState2>, u32>) {}
    fn host_state2_param_variant(_a: imports::HostStateParamVariant) {}
    fn host_state2_param_list(_a: Vec<Handle<HostState2>>) {}

    fn host_state2_result_record() -> imports::HostStateResultRecord {
        imports::HostStateResultRecord { a: Handle::new(HostState2) }
    }
    fn host_state2_result_tuple() -> (Handle<HostState2>,) {
        (Handle::new(HostState2),)
    }
    fn host_state2_result_option() -> Option<Handle<HostState2>> {
        Some(Handle::new(HostState2))
    }
    fn host_state2_result_result() -> Result<Handle<HostState2>, u32> {
        Ok(Handle::new(HostState2))
    }
    fn host_state2_result_variant() -> imports::HostStateResultVariant {
        imports::HostStateResultVariant::V0(Handle::new(HostState2))
    }
    fn host_state2_result_list() -> Vec<Handle<HostState2>> {
        vec![Handle::new(HostState2), Handle::new(HostState2)]
    }
}

wai_bindgen_rust::import!("./tests/runtime/handles/exports.wai");

fn run() -> Result<()> {
    use exports::*;

    exports::test_handle_imports();

    let s: WasmState = wasm_state_create();
    assert_eq!(wasm_state_get_val(&s), 100);
    drop(s);

    assert_eq!(wasm_state2_saw_close(), false);
    let s: WasmState2 = wasm_state2_create();
    assert_eq!(wasm_state2_saw_close(), false);
    drop(s);
    assert_eq!(wasm_state2_saw_close(), true);

    let a = wasm_state_create();
    let b = wasm_state2_create();
    let (s1, s2) = two_wasm_states(&a, &b);
    drop(a);
    drop(s1);
    drop(b);

    wasm_state2_param_record(WasmStateParamRecord { a: &s2 });
    wasm_state2_param_tuple((&s2,));
    wasm_state2_param_option(Some(&s2));
    wasm_state2_param_option(None);
    wasm_state2_param_result(Ok(&s2));
    wasm_state2_param_result(Err(2));
    wasm_state2_param_variant(WasmStateParamVariant::V0(&s2));
    wasm_state2_param_variant(WasmStateParamVariant::V1(2));
    wasm_state2_param_list(&[]);
    wasm_state2_param_list(&[&s2]);
    wasm_state2_param_list(&[&s2, &s2]);
    drop(s2);

    let s = wasm_state2_result_record().a;
    drop(s);
    let s = wasm_state2_result_tuple().0;
    drop(s);
    let s = wasm_state2_result_option().unwrap();
    drop(s);
    let s = wasm_state2_result_result().unwrap();
    match wasm_state2_result_variant() {
        WasmStateResultVariant::V0(s) => drop(s),
        WasmStateResultVariant::V1(_) => panic!(),
    }
    drop(s);
    for s in wasm_state2_result_list() {
        drop(s);
    }
    Ok(())
}
