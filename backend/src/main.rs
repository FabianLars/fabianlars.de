use warp::{Filter, Rejection};

#[tokio::main]
async fn main() {
    let get_champion_file =
        warp::path!("wiki" / "champion").and(warp::fs::file("./champions.json"));

    let champions = warp::path!("wiki" / "champions").and(warp::fs::file("./champions.json"));

    let champion = warp::path!("wiki" / "champions" / u16).and_then(|id: u16| get_champion(id));

    let v1 = warp::path!("v1").and(champions.or(champion));

    warp::serve(get_champion_file.or(v1))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn get_champion(id: u16) -> Result<String, Rejection> {
    let file = tokio::fs::read_to_string("./champions.json")
        .await
        .map_err(|_| warp::reject::not_found())?;
    let json: serde_json::Value =
        serde_json::from_str(&file).map_err(|_| warp::reject::not_found())?;

    if let Some(champ) = json.get(id.to_string()) {
        Ok(champ.to_string())
    } else {
        Err(warp::reject::not_found())
    }
}
