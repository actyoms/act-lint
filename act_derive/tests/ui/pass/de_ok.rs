use act_derive::Deserialize;
#[allow(unused_imports)]
use act_trait::Expecting;
use std::str::FromStr;

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
