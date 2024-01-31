// dev dependencies
use chrono::{Duration, Utc};
use open;
use querystring::querify;
use reqwest::Client;
use std::collections::HashMap;
use tiny_http::{Response, Server};
use tokio_test;

use spotifyrs::authentication;

// FOR TESTS WITH PRINTING: `cargo test -- --nocapture`

// run with feature using: `cargo test --features local_auth`
#[cfg(feature = "local_auth")]
#[test]
fn local_auth_test() {
    let scope = authentication::Scope::new(vec![authentication::Scopes::UserReadPrivate]);
    let client_id = dotenv::var("CLIENT_ID").unwrap(); // grab client_id from .env
    let redirect_port = String::from("8888");

    let request_client = reqwest::Client::new();
    tokio_test::block_on(authentication::local_pkce(
        request_client,
        client_id,
        redirect_port,
        scope,
        60,
    ))
    .unwrap();
}

// #[test]
// fn general_test() {
//     let scope = String::from("playlist-read-private");
//     let redirect_uri = "http://localhost:8888/callback";
//     let client_id = dotenv::var("CLIENT_ID").unwrap(); // grab client_id from .env

//     let (auth_url, pkce_pre_auth) =
//         authentication::pkce_authentication_url(client_id, redirect_uri, scope);

//     // open auth url
//     let open_success = open::that(auth_url);
//     match open_success {
//         Ok(()) => println!("Opened browser successfully"),
//         Err(err) => panic!("{}", err),
//     }

//     // get access code
//     let server = Server::http("127.0.0.1:8888").unwrap();

//     let mut code: Option<String> = None;
//     let mut state: Option<String> = None;

//     let current_time = Utc::now();
//     for request in server.incoming_requests() {
//         let query: HashMap<String, String> = querify(&request.url()[10..])
//             .into_iter()
//             .map(|(x, y)| (String::from(x), String::from(y)))
//             .collect();

//         println!("{:?}", query.keys());

//         // only proceed if we get the code and state
//         if query.contains_key("code") && query.contains_key("state") {
//             let response = Response::from_string("you can close the browser");
//             request.respond(response).unwrap();

//             // extract code and state
//             code = Some(query.get("code").unwrap().clone());
//             state = Some(query.get("state").unwrap().clone());
//             break;
//         }

//         if current_time - Utc::now() > Duration::seconds(60) {
//             panic!("timeout");
//         }
//     }

//     let request_client = Client::new();

//     let pkce = tokio_test::block_on(authentication::new_pkce(
//         request_client,
//         code.unwrap(),
//         state.unwrap(),
//         pkce_pre_auth,
//     ));

//     pkce.unwrap();
// }
