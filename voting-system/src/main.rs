mod vote;
use vote::{Election, Voter};

fn main() {
    let mut election = Election::new(vec!["Alice".to_string(), "Bob".to_string()]);

    election.vote("Alice");
    election.vote("Alice");
    election.vote("Bob");

    for candidate in election.candidates {
        println!("{} has received {} votes", candidate.name, candidate.votes);
    }
}fn main() {
    println!("Hello, world!");
}
