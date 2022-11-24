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
struct ActorPubKey {
    id: String,
    owner: String,
    public_key_pem: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Actor {
    id: String,
    r#type: Actors,
    preferred_username: String,
    inbox: String,
    public_key: ActorPubKey,
}

pub fn actor_response(
    request: Request,
    make_response: &dyn Fn(Request, Response<Cursor<Vec<u8>>>),
) {
    print!("actor");

    let username = dotenv::var(crate::env::USER_ENV_KEY).unwrap();
    let base_url = dotenv::var(crate::env::DOMAIN_ENV_KEY).unwrap();

    let actor_url = format!("{}/{}", base_url, username);

    let actor = Actor {
        id: actor_url.clone(),
        r#type: Actors::Person,
        preferred_username: username,
        inbox: format!("{}/inbox", base_url),
        public_key: ActorPubKey {
            id: format!("{}#main-key", actor_url),
            owner: actor_url,
            public_key_pem: "to_be_confirmed".to_string(),
        },
    };

    // this just shoves the context line above
    // TODO: investigate serde_json to see if there's a nicer way to do this,
    let mut json_actor = dbg!(serde_json::to_string(&actor).unwrap());
    let json_context = format!("\"@context\": [{:?},{:?}],", CONTEXT[0], CONTEXT[1]);
    json_actor.insert_str(1, &json_context);
    make_response(
        request,
        Response::from_string(json_actor)
            .with_header(Header::from_str("Content-Type:application/json;charset=utf-8").unwrap()),
    );
}
