use std::str::FromStr;

use crate::endpoints::{actor::actor_response, well_known::well_known_response};
use dotenv::dotenv;
use tiny_http::{Header, HeaderField, Request, Response, Server, StatusCode};
use url::Url;

pub mod endpoints;
pub mod env;

const DEBUG: bool = false;

fn main() {
    println!("starting");
    dotenv().ok();

    let addr = format!("0.0.0.0:{}", dotenv::var(env::PORT_ENV_KEY).unwrap());

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
            let fq_url = format!("http://{}{}", addr, request.url());
            let url = Url::parse(fq_url.as_str()).unwrap();
            let forwarded_for_header = HeaderField::from_str("X-Forwarded-For").unwrap();
            println!(
                "Host: {:?}",
                request
                    .headers()
                    .into_iter()
                    .find(|header| header.field == forwarded_for_header)
                    .unwrap_or(&Header::from_str("X-Forwarded-For: no-set").unwrap())
            );
            println!("{}", url.path());

            let user = format!("/{}", dotenv::var(env::USER_ENV_KEY).unwrap());
            let path = url.path();
            match path {
                _ if path.contains("/.well-known") => {
                    well_known_response(request, url, &make_response)
                }
                _ if path.contains(&user) => actor_response(request, &make_response),
                _ => make_response(request, Response::empty(StatusCode::from(400))),
            }
        }
    }
}

fn make_response<T>(request: Request, response: Response<T>)
where
    T: std::io::Read,
{
    match request.respond(response) {
        Err(error) => println!("ded {}", error.to_string()),
        Ok(_) => (),
    }
}

// some data structure ot represent different types of request
// some data structure to represent different users
