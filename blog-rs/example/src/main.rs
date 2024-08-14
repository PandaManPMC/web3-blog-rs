use num_bigfloat::{BigFloat, PI};
use num_bigfloat::ONE;

fn main() {
    println!("Hello, world!");


    let six: BigFloat = BigFloat::from_u8(6);
    let three: BigFloat = BigFloat::from_u8(3);
    let pi = six.mul(&ONE.div(&three.sqrt()).atan());
    let epsilon = BigFloat::from_f64(1.0e-38);  // note: conversion from f64,f32 are not loss-less for `no_std`.

    assert!(pi.sub(&PI).abs().cmp(&epsilon).unwrap() < 0);


    println!("Hello, world! 2");

}
