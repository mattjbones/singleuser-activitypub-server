use super::super::types::Follow;

pub fn run(follow: Follow) {
    println!("follow action");
    println!("{:} follows {:}", follow.actor, follow.object);
}
