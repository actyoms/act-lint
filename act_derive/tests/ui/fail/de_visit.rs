use act_derive::Deserialize;
#[allow(unused_imports)]
use act_trait::Expecting;

#[derive(Deserialize)]
#[act(expecting = "something", visit = ["str", "map"])]
struct MissingVisitImpl;

fn main() {}
