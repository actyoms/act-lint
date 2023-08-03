use act_derive::Deserialize;
#[allow(unused_imports)]
use act_trait::Expecting;

#[derive(Deserialize)]
#[act(expecting = "something")]
struct VisitorOrVisit;

fn main() {}
