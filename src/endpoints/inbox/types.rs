use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Signature {
    r#type: String,
    creator: String,
    created: String,
    signature_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteAction {
    id: String,
    actor: String,
    to: Vec<String>,
    object: String,
    signature: Signature,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum InboxAction {
    Delete(DeleteAction),
    //everything else
}
