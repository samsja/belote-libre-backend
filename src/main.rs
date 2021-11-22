use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /info/ => 200 OK with body "Belote Libre"
    let root = warp::path("info")
      .map(|| format!("Belote libre"));


    warp::serve(root)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
