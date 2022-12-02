use super::super::types::Follow;

pub fn run(follow: Follow) {
    println!("{:} follows {:}", follow.actor, follow.object);
}
