use axum::{routing::get, Router};

#[tokio::main]
async fn main(){
    let app = axum::Router::new().route("/", get(hello_world));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> String{
    String::from("hello world")
}