use crate::api::{actor::actor_response, finger::finger_response};
use std::io::Cursor;
use tiny_http::{Request, Response, Server};
use url::Url;

pub mod api;

const PORT: &str = "8000";
const DEBUG: bool = false;

fn main() {
    println!("starting");

    let addr = dbg!(format!("0.0.0.0:{}", PORT));

    let server = Server::http(&addr).unwrap();

    start_server(server, addr);
}

fn start_server(server: Server, addr: String) {
    for request in server.incoming_requests() {
        if DEBUG {
            let response_string = format!(
                "received request! \n\tmethod: {:?}\n\turl: {:?}\n\theaders: {:?}",
                request.method(),
                request.url(),
                request.headers()
            );
            make_response(request, Response::from_string(response_string))
        } else {
            let fq_url = dbg!(format!("http://{}{}", addr, request.url()));
            let url = Url::parse(fq_url.as_str()).unwrap();
            println!("{}", url.path());

            match url.path() {
                "/.well-known/webfinger" => finger_response(request, url, &addr, &make_response),
                "/mbj" => actor_response(request, &addr, &make_response),
                _ => make_response(request, Response::from_string("unknown api")),
            }
        }
    }
}

fn make_response(request: Request, response: Response<Cursor<Vec<u8>>>) {
    match request.respond(response) {
        Err(error) => println!("ded {}", error.to_string()),
        Ok(_) => (),
    }
}

// some data structure ot represent different types of request
// some data structure to represent different users
