use act_derive::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
#[act(visit = ["str"])]
struct MissingExpecting;

impl FromStr for MissingExpecting {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

fn main() {}
