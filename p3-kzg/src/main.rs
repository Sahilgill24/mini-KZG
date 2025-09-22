#![allow(unused)]
use ark_bls12_381::g1::Config;
use ark_bls12_381::{Bls12_381, Fr, FrConfig, G1Affine};
use ark_ec::pairing;
use ark_ec::short_weierstrass::Projective;
use ark_ff::UniformRand;
use ark_ff::{Fp, MontBackend, PrimeField};
use ark_poly::{DenseUVPolynomial, EvaluationDomain, Radix2EvaluationDomain};

/// Represents the group elements for commitment and proof.
struct KZGparams {
    commitment: Projective<Config>,
    proof: G1Affine,
}
fn main() {}

// whole process is very simple
// G1, G2 already there
// [p] calculation has to be done to get the commitment
// then to generate the proof,
