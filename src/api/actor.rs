use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tiny_http::{Request, Response};

const CONTEXT: [&str; 2] = [
    "https://www.w3.org/ns/activitystreams",
    "https://w3id.org/security/v1",
];

const USERNAME: &str = "mbj";

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
    addr: &str,
    make_response: &dyn Fn(Request, Response<Cursor<Vec<u8>>>),
) {
    print!("actor");

    let base_url = format!("http://{}", addr);
    let actor_url = format!("{}/{}", &base_url, USERNAME.to_string());

    let actor = Actor {
        id: actor_url.clone(),
        r#type: Actors::Person,
        preferred_username: USERNAME.to_string(),
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
    make_response(request, Response::from_string(json_actor));
}
