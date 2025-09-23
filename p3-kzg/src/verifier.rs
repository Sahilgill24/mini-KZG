// pairings for verifying the proof generated.

use ark_bls12_381::g1::Config as cfg1;

use ark_bls12_381::g2::Config as cfg;
use ark_bls12_381::{Bls12_381, Config, Fr, FrConfig, G1Affine, G2Affine};
use ark_ec::AffineRepr;
use ark_ec::bls12::Bls12;
use ark_ec::pairing::{Pairing, PairingOutput};
use ark_ec::short_weierstrass::Projective;
use ark_ff::{Fp, MontBackend, PrimeField};

/// e( C- [y]1, H) = e ( pi, [s-z]2)
/// c: commitment
/// [y]1 evaluation at z
/// H: Generator for G2 group
/// pi: proof
/// [s-z]2: s2 - z2 (z implemented at G2Affine group)
pub fn verify_proof(
    evaluation_points: (
        Fp<MontBackend<FrConfig, 4>, 4>,
        Fp<MontBackend<FrConfig, 4>, 4>,
    ),
    commitment: Projective<cfg1>,
    proof: Projective<ark_bls12_381::g1::Config>,
    powers_of_s: (Vec<Projective<cfg1>>, Vec<Projective<cfg>>),
) -> bool {
    let y1 = evaluation_points.1;
    let z = evaluation_points.0;
    let g1 = G1Affine::generator();
    let h = G2Affine::generator();
    let y1_group = g1.mul_bigint(y1.into_bigint());
    let z2_group = h.mul_bigint(z.into_bigint());
    let s2 = powers_of_s.1[1];
    let val: PairingOutput<Bls12<Config>> =
        Bls12::pairing(commitment - y1_group, G2Affine::generator());

    let val2: PairingOutput<Bls12<Config>> = Bls12::pairing(proof, s2 - z2_group);
    val == val2
}
