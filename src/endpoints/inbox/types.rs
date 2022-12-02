use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    r#type: String,
    creator: String,
    created: String,
    signature_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delete {
    pub id: String,
    pub actor: String,
    pub to: Vec<String>,
    pub object: String,
    pub signature: Signature,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Follow {
    pub id: String,
    pub actor: String,
    pub object: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Undo {
    pub id: String,
    pub object: UndoAction,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum UndoAction {
    Follow(Follow),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum InboxAction {
    Delete(Delete),
    Follow(Follow),
    Undo(Undo),
    //everything else
}
