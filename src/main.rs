use warp::Filter;
mod belote;
use belote::Game;


#[tokio::main]
async fn main() {


    let game = Game::new();
    // GET /hello/warp => 200 OK with body "Belote Libre"
    let root = warp::path("info")
      .map(|| format!("Belote libre"));

    let cards = warp::path!("cards"/usize)
      .map(move |player_id| warp::reply::json(&game.clone().get_cards_players(player_id).unwrap()) );



    warp::serve(root.or(cards))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
