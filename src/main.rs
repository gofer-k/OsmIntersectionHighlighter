use seed::{prelude::*, *};
use leaflet::LatLng;

fn main() {
  App::start("app", init, update, view);
}

type Model = i32;

#[derive(Copy, Clone)]
enum Msg {
  Increment
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
  // Cannot initialize Leaflet until the map element has rendered.
  orders.after_next_render(init_map);
  Model::default()
}

fn init_map(_: RenderInfo) {
  let map = leaflet::Map::new("map", &JsValue::NULL);
  map.setView(&LatLng::new(63.5, 10.5), 5.0);    leaflet::TileLayer::new(
      "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",   
      &JsValue::NULL
  ).addTo(&map);
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
  match msg {
    Msg::Increment => *model += 1
  }  
}

fn view(model: &Model) -> Node<Msg> {
  div!(
    "This is my counter ",
    button![model, ev(Ev::Click, |_| Msg::Increment), ]
  )
}