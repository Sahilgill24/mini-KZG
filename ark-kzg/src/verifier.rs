use ark_bls12_381::{Bls12_381, Fr, G1Affine, G1Projective, G2Affine};
use ark_ec::AffineRepr;
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;

use crate::setup::SRS;

/// Verifies that P(z) = y for a committed polynomial P using the pairing equation:
/// e(C - [y]₁, H) = e(pi, [s]₂ - [z]₂)
/// c= [p(s)]1 or the commitment
/// pi = [q(s)]1 or the proof
/// H = G2
/// [y]1 = y.G1
/// [s]2 = second element of the SRS2
/// [z2] = z.G2
pub fn verify_proof(
    z: &Fr,
    y: &Fr,
    commitment: &G1Projective,
    proof: &G1Projective,
    srs: &SRS,
) -> bool {
    let g1 = G1Affine::generator();
    let g2 = G2Affine::generator();

    // [y]₁ = y · G1
    let y_g1 = g1.mul_bigint(y.into_bigint());

    //  [z]₂ = z · G2
    let z_g2 = g2.mul_bigint(z.into_bigint());
    let s_g2 = srs.1[1];

    // Compute the left side of the pairing equation: e(C - [y]₁, H)
    let lhs = Bls12_381::pairing(commitment - y_g1, g2);
    println!("{:?}", lhs);
    // Compute the right side of the pairing equation: e(π, [s]₂ - [z]₂)
    let rhs = Bls12_381::pairing(proof, s_g2 - z_g2);
    println!("{:?}", rhs);
    lhs == rhs
}
