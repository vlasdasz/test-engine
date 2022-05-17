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

    let req = api.request("get_users");

    dbg!(req.call().await.unwrap());

    let mode = req.gotome::<Vec<Model>>().await.unwrap();

    dbg!(mode);

    dbg!("Poka");
}
