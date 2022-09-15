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
        |_, _error| {},
    );

    GET_USERS.get(&(), |_, error, _users| {
        if let Some(_error) = error {
            return;
        }
    });

    loop {
        Dispatch::call()
    }
}
