#![deny(clippy::all)]

use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
};
use napi_derive::napi;
use rdev::{listen, Event, EventType};
use std::{sync::Arc, thread, time::Duration};

mod types;
use types::InputEvent;

#[napi]
pub fn start_root_listener(callback: ThreadsafeFunction<InputEvent>) -> Result<()> {
  let tsfn = Arc::new(callback);

  thread::spawn(move || {
    let send_event = move |event: Event| {
      let new_event = match event.event_type {
        EventType::KeyPress(key) => InputEvent {
          event_type: "KeyDown".to_string(),
          payload: Some(format!("{:?}", key)),
          x: None,
          y: None,
        },
        EventType::KeyRelease(key) => InputEvent {
          event_type: "KeyUp".to_string(),
          payload: Some(format!("{:?}", key)),
          x: None,
          y: None,
        },
        EventType::MouseMove { x, y } => InputEvent {
          event_type: "MouseMove".to_string(),
          payload: None,
          x: Some(x),
          y: Some(y),
        },
        EventType::ButtonPress(button) => InputEvent {
          event_type: "MouseDown".to_string(),
          payload: Some(format!("{:?}", button)),
          x: None,
          y: None,
        },
        EventType::ButtonRelease(button) => InputEvent {
          event_type: "MouseUp".to_string(),
          payload: Some(format!("{:?}", button)),
          x: None,
          y: None,
        },
        _ => return,
      };

      let _ = tsfn.call(Ok(new_event), ThreadsafeFunctionCallMode::NonBlocking);
    };

    if let Err(e) = listen(send_event) {
      eprintln!("Error in rdev listener: {:?}", e);
    }

    thread::sleep(Duration::from_millis(16));
  });

  Ok(())
}
