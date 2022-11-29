use std::{
    io::{Cursor, Empty, Read},
    str::FromStr,
};
use tiny_http::{Header, Request, Response, StatusCode};
use url::Url;

pub fn inbox_response(
    mut request: Request,
    url: Url,
    make_response: &dyn Fn(Request, Response<Empty>),
) {
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content).unwrap();
    println!("{:#?}", content.parse::<String>().unwrap());

    make_response(request, Response::empty(StatusCode::from(200)));
}
