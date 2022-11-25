use dotenv;
use serde::{Deserialize, Serialize};
use std::{fmt::format, io::Cursor, str::FromStr};
use tiny_http::{Header, Request, Response, StatusCode};
use url::Url;

#[derive(Deserialize, Serialize, Debug)]
struct FingerLink {
    rel: String,
    r#type: String,
    href: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Finger {
    subject: String,
    links: Vec<FingerLink>,
}

pub fn well_known_response(
    request: Request,
    url: Url,
    make_response: &dyn Fn(Request, Response<Cursor<Vec<u8>>>),
) {
    print!("well-known");

    let path = url.path();

    match path {
        "/.well-known/webfinger" => webfinger_response(request, url, make_response),
        "/.well-known/nodeinfo" => (),
        _ => (),
    }
}

fn webfinger_response(
    request: Request,
    url: Url,
    make_response: &dyn Fn(Request, Response<Cursor<Vec<u8>>>),
) {
    print!("webfinger");

    let user = dotenv::var(crate::env::USER_ENV_KEY).unwrap();
    let acct_param = url.query_pairs().find_map(|param| {
        if param.0 == "resource" && param.1.contains("acct:") && param.1.contains(&user) {
            Some(param)
        } else {
            None
        }
    });

    if acct_param.is_none() {
        make_response(
            request,
            Response::from_string(format!("no user found: {}", url.path()))
                .with_status_code(StatusCode::from(400)),
        );
        return;
    }

    let username = dotenv::var(crate::env::USER_ENV_KEY).unwrap();
    let base_url = dotenv::var(crate::env::DOMAIN_ENV_KEY).unwrap();
    let actor_url = format!("https://{}/{}", base_url, username);

    let finger_link = FingerLink {
        rel: "self".to_string(),
        r#type: "application/activity+json".to_string(),
        href: actor_url,
    };

    let finger = Finger {
        subject: format!("acct:{}@{}", username, base_url),
        links: vec![finger_link],
    };

    let finger_json = dbg!(serde_json::to_string(&finger).unwrap());
    make_response(
        request,
        Response::from_string(finger_json).with_header(
            Header::from_str("Content-Type:application/jrd+json;charset=utf-8").unwrap(),
        ),
    );
}
