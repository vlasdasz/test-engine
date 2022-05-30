#![feature(explicit_generic_args_with_impl_trait)]

use net::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login:    String,
    password: String,
}

#[tokio::main]
async fn main() {
    dbg!("Helloy");

    let get_users: Request<(), Vec<User>> =
        Request::make("http://ec2-18-217-89-172.us-east-2.compute.amazonaws.com/get_users");
    let register: Request<User, ()> =
        Request::make("http://ec2-18-217-89-172.us-east-2.compute.amazonaws.com/register");

    let users = get_users.get().await.unwrap();
    dbg!(users);

    let _sorekok = get_users.get();

    register
        .post(User {
            login:    "garmanec 2".into(),
            password: "paraguk4ka!".into(),
        })
        .await
        .unwrap();

    dbg!("spisolin");

    let users = get_users.get().await.unwrap();
    dbg!(users);

    dbg!("Poka");
}
