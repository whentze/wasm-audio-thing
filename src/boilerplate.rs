use std::panic;

use wasm_bindgen::{JsCast, convert::FromWasmAbi, prelude::Closure};
use web_sys::{Event, HtmlInputElement};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub(crate) fn init_globals() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
}

pub(crate) trait EventExt {
    fn target_value(self) -> f64;
}

impl EventExt for Event {
    fn target_value(self) -> f64 {
        self.target()
            .unwrap()
            .value_of()
            .unchecked_into::<HtmlInputElement>()
            .value_as_number()
    }
}

pub(crate) fn func<A: FromWasmAbi + 'static, F: FnMut(A) + 'static>(f: F) -> js_sys::Function {
    Closure::wrap(Box::new(f) as Box<dyn FnMut(A)>)
        .into_js_value()
        .unchecked_into()
}