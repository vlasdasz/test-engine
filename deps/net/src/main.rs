#![feature(explicit_generic_args_with_impl_trait)]

use net::API;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login:    String,
    password: String,
}

#[tokio::main]
async fn main() {
    dbg!("Helloy");

    let api = API::new("127.0.0.1:8000");

    let get_users = api.request::<(), Vec<User>>("get_users");
    let register = api.request::<User, ()>("register");

    let users = get_users.get().await.unwrap();
    dbg!(users);

    let sorekok = get_users.get();

    register
        .post(User {
            login:    "garmanec".into(),
            password: "paraguk4ka!".into(),
        })
        .await
        .unwrap();

    dbg!("spisolin");

    let users = get_users.get().await.unwrap();
    dbg!(users);

    dbg!("Poka");
}
