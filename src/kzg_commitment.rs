use std::ops::{Add, Mul, Neg, Sub};
use ark_bls12_381::{Fr, G1Affine, G2Affine};
use ark_poly::univariate::DensePolynomial;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
pub type G1Point = G1Affine;
pub type G2Point = G2Affine;
pub type Poly = DensePolynomial<Fr>;
#[derive(Debug, Clone, PartialEq, Eq, CanonicalDeserialize, CanonicalSerialize)]
pub struct KzgCommitment(pub G1Point);

impl KzgCommitment {
    pub fn inner(&self) -> &G1Point {
        &self.0
    }
}
impl Mul<Fr> for KzgCommitment {
    type Output = Self;

    fn mul(self, rhs: Fr) -> Self::Output {
        let element = self.0.mul(rhs);
        Self(element.into())
    }
}

impl Add for KzgCommitment {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let commitment = self.0 + rhs.0;
        Self(commitment.into())
    }
}

impl Sub for KzgCommitment {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::add(self, -rhs)
    }
}

impl Mul<Fr> for &KzgCommitment {
    type Output = KzgCommitment;

    fn mul(self, rhs: Fr) -> Self::Output {
        let element = self.0.mul(rhs);
        KzgCommitment(element.into())
    }
}

impl Neg for KzgCommitment {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let point = self.0;
        Self(-point)
    }
}