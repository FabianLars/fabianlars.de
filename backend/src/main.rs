use serde::{Deserialize, Serialize};
use warp::Filter;

#[tokio::main]
async fn main() {
    let get_champion_file = warp::path!("wapi" / "champion").and(warp::fs::file("./champions.json"));

    warp::serve(get_champion_file)
        .run(([127, 0, 0, 1], 3030))
        .await;
}