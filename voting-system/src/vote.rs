use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Voter {
    pub id: String,
    pub has_voted: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    pub name: String,
    pub votes: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Election {
    pub candidates: Vec<Candidate>,
}

impl Election {
    pub fn new(candidates: Vec<String>) -> Self {
        Self {
            candidates: candidates.into_iter().map(|name| Candidate { name, votes: 0 }).collect(),
        }
    }

    pub fn vote(&mut self, candidate_name: &str) {
        for candidate in &mut self.candidates {
            if candidate.name == candidate_name {
                candidate.votes += 1;
                break;
            }
        }
    }
}

