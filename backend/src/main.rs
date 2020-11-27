use warp::Filter;

#[tokio::main]
async fn main() {
    let get_champion_file =
        warp::path!("wiki" / "champion").and(warp::fs::file("./champions.json"));

    warp::serve(get_champion_file)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
