#![feature(explicit_generic_args_with_impl_trait)]

use net::{Request, API};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login:    String,
    password: String,
}

#[tokio::main]
async fn main() {
    dbg!("Helloy");

    let _api = API::new("127.0.0.1:8000");

    const GET_USERS: Request<(), Vec<User>> = Request::make("http://127.0.0.1:8000/get_users");
    const REGISTER: Request<User, ()> = Request::make("http://127.0.0.1:8000/register");

    let users = GET_USERS.get().await.unwrap();
    dbg!(users);

    let _sorekok = GET_USERS.get();

    REGISTER
        .post(User {
            login:    "garmanec 2".into(),
            password: "paraguk4ka!".into(),
        })
        .await
        .unwrap();

    dbg!("spisolin");

    let users = GET_USERS.get().await.unwrap();
    dbg!(users);

    dbg!("Poka");
}
