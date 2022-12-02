use super::super::types::Delete;

pub fn run(delete: Delete) {
    println!("delete {:?}", delete.actor);

    //check for sure in database
    //verify signature against stored user
    //delete object - user, toot etc
}
