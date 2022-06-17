use crate::Size;

#[test]
fn nostd_add() {
    let s1 = Size::KiB(12);
    let s2 = Size::KiB(24);
    let sum = s1 + s2;
    assert_eq!(sum.bytes(), Size::KiB(36).bytes());
}

#[test]
fn nostd_sub() {
    let s1 = Size::KiB(24_i32);
    let s2 = Size::KiB(12_i64);
    let sum = s1 - s2;
    assert_eq!(sum.bytes(), Size::KiB(12).bytes());
}

#[test]
fn nostd_neg_sub() {
    let s1 = Size::KiB(12_u64);
    let s2 = Size::KiB(24_i64);
    let sum = s1 - s2;
    assert_eq!(sum.bytes(), Size::KiB(-12).bytes());
}

#[test]
fn nostd_bytes() {
    let s1 = Size::KiB(36);
    let s2 = Size::Bytes(36<<10);
    assert_eq!(s1.bytes(), s2.bytes());
    assert_eq!(s1.bytes(), 36<<10);
}
