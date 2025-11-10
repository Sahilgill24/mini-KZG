use ark_bls12_381::{Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::AffineRepr;
use ark_ff::{PrimeField, UniformRand};
use ark_poly::univariate::DensePolynomial;
use ark_poly::{DenseUVPolynomial, Polynomial};

pub type PolyCoeffs = Vec<Fr>;
pub type Secret = Fr;
pub type SRS1 = Vec<G1Projective>;
pub type SRS2 = Vec<G2Projective>;
pub type SRS = (SRS1, SRS2);

/// Generates a random secret for the trusted setup.
pub fn generate_secret() -> Secret {
    let mut rng = ark_std::test_rng();
    Fr::rand(&mut rng)
}

/// Generate the structured reference string (SRS) for KZG commitments.
/// Computes [s^i]₁ , [s^i]2 where  G1 and G2 are the respective groups.
/// [s^i]G1 and [s^i]G2
/// where s is the secret and the brackets denote scalar multiplication.
pub fn powers_of_s(n: usize, secret: Secret) -> SRS {
    let g1 = G1Affine::generator();
    let g2 = G2Affine::generator();

    let mut srs1 = Vec::with_capacity(n);
    let mut srs2 = Vec::with_capacity(n);
    let mut power = Fr::from(1u64);

    for _ in 0..n {
        srs1.push(g1.mul_bigint(power.into_bigint()));
        srs2.push(g2.mul_bigint(power.into_bigint()));
        power *= secret;
    }

    (srs1, srs2)
}

/// Generates a random polynomial of the specified degree.
pub fn generate_polynomial(degree: usize) -> PolyCoeffs {
    let mut rng = ark_std::test_rng();
    let poly = DensePolynomial::rand(degree, &mut rng);
    poly.coeffs
}

/// Evaluate a polynomial at a given point z.
/// y = P(z)
pub fn evaluate_polynomial(polynomial: &PolyCoeffs, z: &Fr) -> (Fr, Fr) {
    let poly = DensePolynomial::from_coefficients_slice(polynomial);
    let y = poly.evaluate(z);
    (*z, y)
}
