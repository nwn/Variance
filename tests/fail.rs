#[test]
fn failure_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail_covariant.rs");
    t.compile_fail("tests/fail_contravariant.rs");
    t.compile_fail("tests/fail_invariant_covariant.rs");
    t.compile_fail("tests/fail_invariant_contravariant.rs");
}
