use ark_bls12_381::Fr;
use ark_kzg::error::Result;
use ark_kzg::prover::{generate_commitment, generate_proof};
use ark_kzg::setup::{evaluate_polynomial, generate_polynomial, generate_secret, powers_of_s};
use ark_kzg::verifier::verify_proof;

fn main() -> Result<()> {
    let max_degree = 8;
    let srs_size = max_degree + 1;

    let secret = generate_secret();
    let srs = powers_of_s(srs_size, secret);

    let polynomial = generate_polynomial(max_degree);

    let commitment = generate_commitment(&polynomial, &srs)?;

    let z = Fr::from(23u64);
    let (eval_point, eval_value) = evaluate_polynomial(&polynomial, &z);

    let proof = generate_proof(&polynomial, &srs, &eval_point, &eval_value)?;

    let is_valid = verify_proof(&eval_point, &eval_value, &commitment, &proof, &srs);
    println!("{}", is_valid);

    Ok(())
}
