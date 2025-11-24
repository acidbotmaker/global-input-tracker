use napi_derive::napi;

#[napi(object)]
pub struct InputEvent {
  pub event_type: String,
  pub payload: Option<String>,
  pub x: Option<f64>,
  pub y: Option<f64>,
}
