mod synth;
use synth::FmOsc;

mod boilerplate;
use boilerplate::{func, init_globals, EventExt};

use std::cell::RefCell;
use web_sys::Event;

fn main() {
    init_globals();

    let document = web_sys::window().and_then(|win| win.document()).unwrap();

    let fm: &'static RefCell<Option<FmOsc>> = Box::leak(Box::new(RefCell::new(None)));

    document
        .get_element_by_id("play")
        .unwrap()
        .add_event_listener_with_callback(
            "click",
            &func(move |_: Event| {
                let mut o = fm.borrow_mut();
                if o.is_none() {
                    let mut osc = FmOsc::new().unwrap();
                    osc.set_note(50);
                    osc.set_fm_frequency(0.0);
                    osc.set_fm_amount(0.0);
                    *o = Some(osc);
                } else {
                    *o = None;
                }
            }),
        )
        .unwrap();

    document
        .get_element_by_id("primary_input")
        .unwrap()
        .add_event_listener_with_callback(
            "input",
            &func(move |e: Event| {
                fm.borrow_mut()
                    .as_mut()
                    .map(|f| f.set_note(e.target_value() as u8));
            }),
        )
        .unwrap();

    document
        .get_element_by_id("fm_freq")
        .unwrap()
        .add_event_listener_with_callback(
            "input",
            &func(move |e: Event| {
                fm.borrow_mut()
                    .as_mut()
                    .map(|f| f.set_fm_frequency(e.target_value() as f32));
            }),
        )
        .unwrap();

    document
        .get_element_by_id("fm_amount")
        .unwrap()
        .add_event_listener_with_callback(
            "input",
            &func(move |e: Event| {
                fm.borrow_mut()
                    .as_mut()
                    .map(|f| f.set_fm_amount(e.target_value() as f32));
            }),
        )
        .unwrap();
}
