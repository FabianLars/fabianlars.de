use chrono::NaiveDate;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use warp::{Filter, Rejection};

#[derive(Serialize)]
struct Rotation {
    rotation_id: u16,
    start_date: NaiveDate,
    end_date: NaiveDate,
    rotation: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    let rotations = warp::path!("lol" / "rotations").and_then(|| async {
        let rows = sqlx::query_as!(Rotation, "SELECT * FROM rotations")
            .fetch_all(&pool)
            .await?;

        Ok(rows)
    });

    let rotation = warp::path!("lol" / "rotations" / u16).and_then(|id: u16| async {
        let row = sqlx::query_as!(
            Rotation,
            "SELECT * FROM rotations WHERE rotation_id = ?",
            id,
        )
        .fetch_one(&pool)
        .await?;

        Ok(row)
    });

    let rotation_latest = warp::path!("lol" / "rotations" / "latest").and_then(|| async {
        let row = sqlx::query_as!(
            Rotation,
            "SELECT * FROM rotations ORDER BY rotation_id DESC LIMIT 1",
            id,
        )
        .fetch_one(&pool)
        .await?;

        Ok(row)
    });

    let champions = warp::path!("lol" / "champions").and(warp::fs::file("./champions.json"));

    let champion = warp::path!("lol" / "champions" / u16).and_then(get_champion);

    // OLD
    let champions_old = warp::path!("wiki" / "champions").and(warp::fs::file("./champions.json"));
    let champion_old = warp::path!("wiki" / "champions" / u16).and_then(get_champion);
    // OLD END

    let v1 = warp::path!("v1" / ..).and(
        champions_old
            .or(champion_old)
            .or(champions)
            .or(champion)
            .or(rotations)
            .or(rotation)
            .or(rotation_latest),
    );

    warp::serve(v1).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
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
