use serde::{Deserialize, Serialize};
use std::{io::Cursor, str::FromStr};
use tiny_http::{Header, Request, Response};

const CONTEXT: [&str; 2] = [
    "https://www.w3.org/ns/activitystreams",
    "https://w3id.org/security/v1",
];

#[derive(Deserialize, Serialize, Debug)]
enum Actors {
    Person,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ActorPubKey {
    id: String,
    owner: String,
    public_key_pem: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Actor {
    id: String,
    r#type: Actors,
    preferred_username: String,
    summary: String,
    inbox: String,
    public_key: ActorPubKey,
}

pub fn actor_handler(request: Request, make_response: &dyn Fn(Request, Response<Cursor<Vec<u8>>>)) {
    print!("actor");

    let username = dotenv::var(crate::env::USER_ENV_KEY).unwrap();
    let domain = dotenv::var(crate::env::DOMAIN_ENV_KEY).unwrap();

    let base_url = format!("https://{}", domain);
    let actor_url = format!("{}/{}", base_url, username);

    let actor = Actor {
        id: actor_url.clone(),
        r#type: Actors::Person,
        preferred_username: username,
        summary: "testing".to_string(),
        inbox: format!("{}/inbox", base_url),
        public_key: ActorPubKey {
            id: format!("{}#main-key", actor_url),
            owner: actor_url,
            public_key_pem: dotenv::var(crate::env::PEM_ENV_KEY).unwrap(),
        },
    };

    // this just shoves the context line above
    // TODO: investigate JSON-LD to see if there's a nicer way to do this,
    let mut json_actor = serde_json::to_string(&actor).unwrap();
    let json_context = format!("\"@context\": [{:?},{:?}],", CONTEXT[0], CONTEXT[1]);
    json_actor.insert_str(1, &json_context);
    make_response(
        request,
        Response::from_string(json_actor).with_header(
            Header::from_str("Content-Type:application/jrd+json;charset=utf-8").unwrap(),
        ),
    );
}
