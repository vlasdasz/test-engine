#![feature(explicit_generic_args_with_impl_trait)]

use net::API;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Model {
    login:    String,
    password: String,
}

#[tokio::main]
async fn main() {
    dbg!("Helloy");

    let api = API::new("127.0.0.1:8000");

    let req = api.request::<Vec<Model>>("get_users");

    let mode = req.gotome().await.unwrap();

    dbg!(mode);

    dbg!("Poka");
}
