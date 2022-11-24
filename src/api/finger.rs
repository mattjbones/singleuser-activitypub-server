use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tiny_http::StatusCode;
use tiny_http::{Request, Response};
use url::Url;

const USERNAME: &str = "mbj";

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

pub fn finger_response(
    request: Request,
    url: Url,
    addr: &str,
    make_response: &dyn Fn(Request, Response<Cursor<Vec<u8>>>),
) {
    print!("finger");

    let acct_param = url.query_pairs().find_map(|param| {
        if param.0 == "resource" && param.1.contains("acct:") {
            Some(param)
        } else {
            None
        }
    });

    if acct_param.is_none() {
        make_response(
            request,
            Response::from_string("no user found, acct:").with_status_code(StatusCode::from(400)),
        );
        return;
    }

    let base_url = format!("http://{}", addr);
    let actor_url = format!("{}/{}", &base_url, USERNAME.to_string());

    let finger_link = FingerLink {
        rel: "self".to_string(),
        r#type: "application/activity+json".to_string(),
        href: actor_url,
    };

    let finger = Finger {
        subject: format!("acct:{}@{}", USERNAME.to_string(), addr),
        links: vec![finger_link],
    };

    let finger_json = dbg!(serde_json::to_string(&finger).unwrap());
    make_response(request, Response::from_string(finger_json));
}
