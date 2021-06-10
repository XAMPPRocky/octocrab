#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/01-valid.rs");
    t.compile_fail("tests/ui/02-no-struct.rs");
    t.compile_fail("tests/ui/03-no-fields.rs");
}