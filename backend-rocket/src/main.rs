#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rust_embed;

use serde::{Deserialize, Serialize};
use rocket::http::{ContentType, Status};
use rocket::response;
use std::ffi::OsStr;
use std::io::Cursor;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "../frontend-preact/dist"]
struct Asset;

#[derive(Serialize, Deserialize, Debug)]
struct Champ {
    id: u32,
    isBase: Option<bool>,
    name: String,
    splashPath: Option<String>,
    uncenteredSplashPath: Option<String>,
    tilePath: Option<String>,
    loadScreenPath: Option<String>,
    skinType: Option<String>,
    rarity: Option<String>,
    isLegacy: bool,
    splashVideoPath: Option<String>,
    featuresText: Option<String>,
    chromaPath: Option<String>,
    emblems: Option<String>,
    regionRarityId: u32,
    rarityGemPath: Option<String>,
    skinLines: Option<Vec<HashMap<String, u32>>>,
    description: Option<String>,
}

#[get("/")]
fn index<'r>() -> response::Result<'r> {
  Asset::get("index.html").map_or_else(
    || Err(Status::NotFound),
    |d| response::Response::build().header(ContentType::HTML).sized_body(Cursor::new(d)).ok(),
  )
}

#[get("/<file..>", rank = 10)]
fn dist<'r>(file: PathBuf) -> response::Result<'r> {
  let filename = file.display().to_string();
  Asset::get(&filename).map_or_else(
    || Err(Status::NotFound),
    |d| {
      let ext = file
        .as_path()
        .extension()
        .and_then(OsStr::to_str)
        .ok_or(Status::new(400, "Could not get file extension"))?;
      let content_type = ContentType::from_extension(ext).ok_or(Status::new(400, "Could not get file content type"))?;
      response::Response::build().header(content_type).sized_body(Cursor::new(d)).ok()
    },
  )
}

#[get("/api/re/<skin>")]
fn re(skin: String) -> response::Redirect {
    let mut skin_id: Option<String> = None;
    let response_text = reqwest::blocking::get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skins.json")
        .expect("Could not make request to CommunityDragon")
        .text().expect("Could not read response text!");

    let skins_map: HashMap<String, Champ> = serde_json::from_str(&response_text).expect("Could not convert skin-data to Vector");

    for (_key, value) in &skins_map {
        if value.name == skin {
            skin_id = Some(value.id.to_string());
            break;
        }
    }

    if let Some(value) = skin_id {
        response::Redirect::to(format!("{}{}", "https://teemo.gg/model-viewer?skinid=", value))
    } else {
        response::Redirect::to("https://teemo.gg/model-viewer?skinid=")
    }
}

fn main() {
  rocket::ignite()
  .mount("/", routes![index, dist, re])
  .launch();
}
