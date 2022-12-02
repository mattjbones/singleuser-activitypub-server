use std::{
    str::FromStr,
    sync::Arc,
    thread::{self, JoinHandle},
};

use crate::endpoints::{
    actor::actor_handler, inbox::inbox_handler, well_known::well_known_handler,
};
use dotenv::dotenv;
use tiny_http::{Header, HeaderField, Request, Response, Server, StatusCode};
use url::Url;

pub mod endpoints;
pub mod env;

const DEFAULT_WORKERS: usize = 4;

fn main() {
    println!("starting");
    dotenv().ok();

    let addr = format!("0.0.0.0:{}", dotenv::var(env::PORT_ENV_KEY).unwrap());
    let workers = dotenv::var(env::SERVER_WORKERS_KEY)
        .unwrap_or(DEFAULT_WORKERS.to_string())
        .parse::<usize>()
        .unwrap();

    let server = Server::http(&addr).unwrap();

    start_workers(server, addr, workers);
}

fn start_workers(server: Server, addr: String, workers: usize) {
    let server = Arc::new(server);
    let mut guards: Vec<JoinHandle<()>> = Vec::with_capacity(workers);

    for _ in 0..workers {
        let fq_domain = format!("http://{}", addr);
        let server = server.clone();
        let guard = thread::spawn(move || loop {
            let request = server.recv().unwrap();
            handle_request(request, &fq_domain);
        });
        guards.push(guard);
    }

    for guard in guards {
        guard.join().unwrap();
    }

    // guards.get(0).unwrap().join().expect("threads");

    /*
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
                    .unwrap_or(&Header::from_str("X-Forwarded-For: not-set").unwrap())
            );
            println!("{}", url.path());

            let user = format!("/{}", dotenv::var(env::USER_ENV_KEY).unwrap());
            let path = url.path();
            match path {
                _ if path.contains("/.well-known") => {
                    well_known_handler(request, url, &make_response)
                }
                _ if path.contains(&user) => actor_handler(request, &make_response),
                "/inbox" => inbox_handler(request, &make_response),
                _ => make_response(request, Response::empty(StatusCode::from(400))),
            }
        }
    }
    */
}

fn handle_request(request: Request, fq_domain: &String) {
    let fq_url = format!("{}{}", fq_domain, request.url());
    let url = Url::parse(fq_url.as_str()).unwrap();
    let forwarded_for_header = HeaderField::from_str("X-Forwarded-For").unwrap();
    println!(
        "Host: {:?}",
        request
            .headers()
            .into_iter()
            .find(|header| header.field == forwarded_for_header)
            .unwrap_or(&Header::from_str("X-Forwarded-For: not-set").unwrap())
    );
    println!("{}", url.path());

    let user = format!("/{}", dotenv::var(env::USER_ENV_KEY).unwrap());
    let path = url.path();
    match path {
        _ if path.contains("/.well-known") => well_known_handler(request, url, &make_response),
        _ if path.contains(&user) => actor_handler(request, &make_response),
        "/inbox" => inbox_handler(request, &make_response),
        _ => make_response(request, Response::empty(StatusCode::from(400))),
    }
}

fn make_response<T>(request: Request, response: Response<T>)
where
    T: std::io::Read,
{
    match request.respond(response.with_header(Header::from_str("Server: sassy").unwrap())) {
        Err(error) => println!("ded {}", error.to_string()),
        Ok(_) => (),
    }
}

// some data structure ot represent different types of request
// some data structure to represent different users
