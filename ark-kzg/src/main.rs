use ark_bls12_381::G1Affine;
use ark_bls12_381::g1::Config;
use ark_ec::short_weierstrass::Projective;

use ark_kzg::prover::{generate_commitment, generate_proof};
use ark_kzg::setup::{evaluate_polynomial, generate_polynomial, generate_secret, powers_of_s};
use ark_kzg::verifier::verify_proof;

/// Represents the group elements for commitment and proof.
struct KZGparams<'a> {
    commitment: Projective<Config>,
    proof: &'a G1Affine,
}

fn main() {
    let n = 9;
    let secret = generate_secret();
    let polynomial = generate_polynomial(n);
    let polynomial2 = polynomial.clone();
    let p3 = polynomial.clone();
    let powers_of_s = powers_of_s(n, secret);
    let ps2 = powers_of_s.clone();
    let ps3 = powers_of_s.clone();
    let commitment = generate_commitment(polynomial, powers_of_s);
    let evaluation_points = evaluate_polynomial(polynomial2, 23);
    let proof = generate_proof(p3, ps2, evaluation_points);
    let val = verify_proof(evaluation_points, commitment, proof, ps3);
    println!("{:?}", commitment);
    println!("{:?}", secret);
    println!("{:?}", evaluation_points);
    println!("{:?}", proof);
    println!("{:?}", val);
}

// whole process is very simple
// G1, G2 already there
// [p] calculation has to be done to get the commitment
// then to generate the proof,
