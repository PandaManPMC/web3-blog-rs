use num_bigfloat::{BigFloat, PI};
use num_bigfloat::ONE;

#[tokio::main]
async fn main() {
    println!("Hello, world!");


    let six: BigFloat = BigFloat::from_u8(6);
    let three: BigFloat = BigFloat::from_u8(3);
    let pi = six.mul(&ONE.div(&three.sqrt()).atan());
    let epsilon = BigFloat::from_f64(1.0e-38);  // note: conversion from f64,f32 are not loss-less for `no_std`.

    assert!(pi.sub(&PI).abs().cmp(&epsilon).unwrap() < 0);


    println!("Hello, world! 2");

    let amount = "0.01".to_string();
    let u = "1.2".to_string();
    let a = BigFloat::parse(amount.as_str()).unwrap();
    let b = BigFloat::parse(u.as_str()).unwrap();

    let p = a.mul(&b);
    println!("{:?}",format!("{:.10}", p.to_f64()));

    let r = common::tool::contract::get_address("abc".to_string()).await;
    println!("{:?}", r)

}
