use super::build_test;
use rand_utils::rand_value;

#[test]
fn add_unsafe() {
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = a.wrapping_add(b);

    let source = "
        use.std::math::u64
        begin
            exec.u64::add_unsafe
        end";

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    let (c1, c0) = split_u64(c);

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_stack(&[c1, c0]);
}

#[test]
fn mul_unsafe() {
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = a.wrapping_mul(b);

    let source = "
        use.std::math::u64
        begin
            exec.u64::mul_unsafe
        end";

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    let (c1, c0) = split_u64(c);

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_stack(&[c1, c0]);
}

#[test]
fn div_unsafe() {
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = a / b;

    let source = "
        use.std::math::u64
        begin
            exec.u64::div_unsafe
        end";

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    let (c1, c0) = split_u64(c);

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_stack(&[c1, c0]);

    let d = a / b0;
    let (d1, d0) = split_u64(d);

    let test = build_test!(source, &[a0, a1, b0, 0]);
    test.expect_stack(&[d1, d0]);
}

// HELPER FUNCTIONS
// ================================================================================================

/// Split the provided u64 value into 32 hight and low bits.
fn split_u64(value: u64) -> (u64, u64) {
    (value >> 32, value as u32 as u64)
}