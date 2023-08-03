use trybuild::TestCases;

#[test]
fn failures() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
}

#[test]
fn passes() {
    let t = TestCases::new();
    t.pass("tests/ui/pass/*.rs");
}
