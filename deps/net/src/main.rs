#![feature(explicit_generic_args_with_impl_trait)]

use net::{GetRequest, PostRequest, API};
use rtools::Dispatch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login:    String,
    password: String,
}

#[tokio::main]
async fn main() {
    dbg!("Helloy");

    // const API: API =
    // API::new("ec2-18-217-89-172.us-east-2.compute.amazonaws.com");
    const API: API = API::new("127.0.0.1");

    const GET_USERS: GetRequest<Vec<User>> = API.get("get_users");
    const REGISTER: PostRequest<User> = API.post("register");

    REGISTER.post(
        User {
            login:    "garmanec 5".into(),
            password: "paraguk4ka!".into(),
        },
        &(),
        |_, error| {
            dbg!(error);
        },
    );

    GET_USERS.get(&(), |_, error, users| {
        if let Some(error) = error {
            dbg!(error);
            return;
        }

        dbg!(users);
    });

    loop {
        Dispatch::call()
    }
}
