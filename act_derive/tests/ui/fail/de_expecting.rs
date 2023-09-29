use std::str::FromStr;

use act_derive::Deserialize;

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
