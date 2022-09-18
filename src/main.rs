// External dependencies
use seed::{prelude::*, *};
use leaflet::{LatLng, Polyline};
use serde::{Serialize, Deserialize};

use quick_xml::de;

// Internal dependencies
mod osm;

fn main() {
  App::start("app", init, update, view);
}

struct Model {
  map: Option<leaflet::Map>,
  osm_doc: Option<osm::OsmDocument>,
}

#[derive(Serialize, Deserialize)]
struct PolylineOptions {
    color: String,
    weight: u32,
}

enum Msg {
  OsmFetched(fetch::Result<String>),
  Map(leaflet::Map)
}

// Custom helpers start here
fn get_osm_request_url() -> &'static str {
  "https://www.openstreetmap.org/api/0.6/map?bbox=10.2%2C63.4%2C10.3%2C63.4"
}

async fn send_osm_request() -> fetch::Result<String> {
  fetch(get_osm_request_url())
      .await?
      .check_status()?
      .text()
      .await
}

impl From<&osm::OsmNode> for LatLng {
  fn from(osm_node: &osm::OsmNode) -> Self {
    LatLng::new(osm_node.lat, osm_node.lon)
  }
}

fn render_topology(model: &Model) {
  if let Some(map) = &model.map {
    if let Some(osm) = &model.osm_doc {
      for way in osm.ways.iter() {
        Polyline::new_with_options(
            way.points(&osm)
                .into_iter()
                .map(LatLng::from)
                .map(JsValue::from)
                .collect(),
            &JsValue::from_serde(&PolylineOptions {
                color: "blue".into(),
                weight: 2,
            })            
            .expect("Unable to serialize polyline options"),
        )
        .addTo(&map);
      }
    }
  }
}

// Seed callbacks start here
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
  // Cannot initialize Leaflet until the map element has rendered.
  orders.after_next_render(move |_| {
    let map = leaflet::Map::new("map", &JsValue::NULL);    
    Msg::Map(map)
  });    

  orders.perform_cmd(async {      
    Msg::OsmFetched(send_osm_request().await) 
  });

  Model {
    map: None, // map will be set to model in 'update' callback. Here is default initialization
    osm_doc: None,
  }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
  match msg {
    Msg::OsmFetched(Ok(response_data)) => {
      model.osm_doc = Some(quick_xml::de::from_str(&response_data).expect("Unable to deserialize the OSM data"));      
      render_topology(&model);
    }
    Msg::OsmFetched(Err(fetch_error)) => {
      error!("Fetching OSM data failed: {:#?}", fetch_error);
    }
    Msg::Map(map) => {      
      map.setView(&LatLng::new(63.5, 10.5), 5.0);    leaflet::TileLayer::new(
        "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",   
        &JsValue::NULL
      ).addTo(&map);
      model.map = Some(map);
    }
  }
}

fn view(model: &Model) -> Node<Msg> {
  if let Some(osm) = &model.osm_doc {
    div![
      div![id!["map"]],
      div![osm.ways.iter().map(view_way)],
    ]
  }
  else {
    div![
      div![id!["map"]],
      text!["No OSm way data available"],
    ]
  }
}

fn view_way(way: &osm::OsmWay) -> Node<Msg> {
  div![
      h2![&way.id],
      ul![way
          .tags
          .iter()
          .map(|tag| li![format!("{} = {}", tag.k, tag.v)])]
  ]
}