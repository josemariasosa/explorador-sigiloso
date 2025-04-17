mod btc;

use axum::{
    routing::get,
    Router,
};
// use std::net::SocketAddr;

#[tokio::main]
async fn main() {

    // build our application with a single route
    let app = Router::new().route("/btc/balance/{address}", get(btc::get_balance));

    // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Explorador Sigiloso API running at http://{:?}/", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();





    // let app = Router::new()
    //     .route("/btc/balance/:address", get(btc::get_balance));

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
