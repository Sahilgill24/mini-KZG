#![allow(unused)]
use ark_bls12_381::g1::Config;
use ark_bls12_381::{Bls12_381, Fr, FrConfig, G1Affine};
use ark_ec::pairing;
use ark_ec::short_weierstrass::Projective;
use ark_ff::UniformRand;
use ark_ff::{Fp, MontBackend, PrimeField};
use ark_poly::{DenseUVPolynomial, EvaluationDomain, Radix2EvaluationDomain};
use p3_kzg::prover::generate_commitment;
use p3_kzg::setup::{generate_polynomial, generate_secret, powers_of_s};

/// Represents the group elements for commitment and proof.
struct KZGparams {
    commitment: Projective<Config>,
    proof: G1Affine,
}
fn main() {
    let n = 9;
    let secret = generate_secret();
    let polynomial = generate_polynomial(n);
    let powers_of_s = powers_of_s(n, secret);
    let commitment = generate_commitment(polynomial, powers_of_s);
    println!("{:?}", commitment);
    println!("{:?}", secret);
}

// whole process is very simple
// G1, G2 already there
// [p] calculation has to be done to get the commitment
// then to generate the proof,
