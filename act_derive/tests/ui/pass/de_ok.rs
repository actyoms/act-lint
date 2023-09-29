use std::str::FromStr;

use act_derive::Deserialize;

#[derive(Deserialize)]
#[act(expecting = "something", visit = ["str"])]
struct IsOk;

impl FromStr for IsOk {
    type Err = std::io::Error;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

fn main() {}
