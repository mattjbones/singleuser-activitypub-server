use std::io::Empty;
use tiny_http::{Header, Request, Response, StatusCode};
use types::*;

pub mod actions;
pub mod types;

fn verify_signature(headers: &[Header]) -> bool {
    let signature_header = headers
        .into_iter()
        .find(|header| header.field.as_str() == "Signature");
    signature_header.is_some()
}

pub fn inbox_handler(mut request: Request, make_response: &dyn Fn(Request, Response<Empty>)) {
    println!("inbox");

    if !verify_signature(request.headers()) {
        println!("No signature");
        make_response(request, Response::empty(StatusCode::from(401)));
        return;
    }

    let mut content = String::new();
    request.as_reader().read_to_string(&mut content).unwrap();

    let action = match serde_json::from_str::<InboxAction>(&content.as_str()) {
        Ok(action) => Some(action),
        Err(err) => {
            println!("{}", err);
            println!("{}", content.as_str());
            None
        }
    };

    if let Some(some_action) = action {
        match some_action {
            InboxAction::Delete(delete) => actions::delete::run(delete),
            InboxAction::Follow(follow) => actions::follow::run(follow),
            InboxAction::Undo(undo) => actions::undo::run(undo),
        }
    }

    make_response(request, Response::empty(StatusCode::from(200)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_verify_valid_signature() {
        let headers = [Header::from_str("Signature: test").unwrap()];
        assert_eq!(verify_signature(&headers), true);
    }

    #[test]
    fn test_verify_invalid_signature() {
        let headers = [Header::from_str("Location: test").unwrap()];
        assert_eq!(verify_signature(&headers), false);
    }
}
