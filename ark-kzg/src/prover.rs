use ark_bls12_381::{Fr, G1Projective};
use ark_ec::PrimeGroup;
use ark_ff::{PrimeField, Zero};
use ark_poly::DenseUVPolynomial;
use ark_poly::univariate::{DenseOrSparsePolynomial, DensePolynomial};

use crate::error::{KzgError, Result};
use crate::setup::{PolyCoeffs, SRS};

/// Generates the commitment
/// c = [P(s)]1
/// c = summation pi. [s^i]G1
pub fn generate_commitment(polynomial: &PolyCoeffs, srs: &SRS) -> Result<G1Projective> {
    // [s^i]G1
    let srs1 = &srs.0;

    if polynomial.len() > srs1.len() {
        return Err(KzgError::PolynomialLengthMismatch {
            polynomial_len: polynomial.len(),
            powers_len: srs1.len(),
        });
    }

    let commitment = polynomial
        .iter()
        .zip(srs1.iter())
        .map(|(coeff, power)| power.mul_bigint(coeff.into_bigint()))
        .sum();

    Ok(commitment)
}

/// Generate a KZG evaluation proof.
/// Computes the quotient polynomial q(x) = (P(x) - P(z)) / (x - z)
/// and returns the proof [q(s)]₁.
pub fn generate_proof(polynomial: &PolyCoeffs, srs: &SRS, z: &Fr, y: &Fr) -> Result<G1Projective> {
    let srs1 = &srs.0;

    // P(x)- P(z)
    let mut adjusted_poly = polynomial.clone();
    adjusted_poly[0] -= y;

    // x-z
    let divisor = DensePolynomial::from_coefficients_vec(vec![-*z, Fr::from(1u64)]);

    // represent P(x) - P(z) as a polynomial
    let numerator = DensePolynomial::from_coefficients_vec(adjusted_poly);

    // q(x) = (P(x) - P(z)) / (x - z)
    let division_result =
        DenseOrSparsePolynomial::divide_with_q_and_r(&numerator.into(), &divisor.into());

    let (quotient, remainder) = division_result.ok_or(KzgError::PolynomialDivisionFailed)?;

    // All the coeffecients of the remainder should be zero 
    if remainder.coeffs.iter().any(|c| !c.is_zero()) {
        return Err(KzgError::PolynomialDivisionFailed);
    }

    let quotient_coeffs = quotient.coeffs;

    // proof [q(s)]1
    // summation qi . [s^i]G1
    let proof = quotient_coeffs
        .iter()
        .zip(srs1.iter())
        .map(|(coeff, power)| power.mul_bigint(coeff.into_bigint()))
        .sum();

    Ok(proof)
}
