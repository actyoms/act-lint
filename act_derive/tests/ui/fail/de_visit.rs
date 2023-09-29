use act_derive::Deserialize;

#[derive(Deserialize)]
#[act(expecting = "something", visit = ["str", "map", "number", "bool"])]
struct MissingVisitImpl;

fn main() {}
