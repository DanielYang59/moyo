use nalgebra::base::{Matrix3, Vector3};
use std::ops::Mul;

use super::lattice::Lattice;

pub type Rotation = Matrix3<i32>;
pub type Translation = Vector3<f64>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RotationType {
    Rotation1,      // 1
    Rotation2,      // 2
    Rotation3,      // 3
    Rotation4,      // 4
    Rotation6,      // 6
    RotoInversion1, // -1 = S2
    RotoInversion2, // -2 = m = S1
    RotoInversion3, // -3 = S6^-1
    RotoInversion4, // -4 = S4^-1
    RotoInversion6, // -6 = S3^-1
}

#[derive(Debug, PartialEq, Clone)]
pub struct Permutation {
    pub mapping: Vec<usize>,
}

#[derive(Debug)]
pub struct Operations {
    pub lattice: Lattice,
    //
    pub rotations: Vec<Rotation>,
    pub translations: Vec<Translation>,
}

impl Operations {
    pub fn new(lattice: Lattice, rotations: Vec<Rotation>, translations: Vec<Translation>) -> Self {
        if translations.len() != rotations.len() {
            panic!("rotations and translations should be the same length");
        }
        Self {
            lattice,
            rotations,
            translations,
        }
    }

    pub fn num_operations(&self) -> usize {
        self.rotations.len()
    }

    pub fn cartesian_rotations(&self) -> Vec<Matrix3<f64>> {
        let inv_basis = self.lattice.basis.try_inverse().unwrap();
        self.rotations
            .iter()
            .map(|r| self.lattice.basis * r.map(|e| e as f64) * inv_basis)
            // .map(|r| inv_basis * r.map(|e| e as f64) * self.lattice.basis)
            .collect()
    }
}

impl Permutation {
    pub fn new(mapping: Vec<usize>) -> Self {
        Self { mapping }
    }

    pub fn identity(size: usize) -> Self {
        Self::new((0..size).collect())
    }

    pub fn size(&self) -> usize {
        self.mapping.len()
    }

    pub fn apply(&self, i: usize) -> usize {
        self.mapping[i]
    }

    pub fn inverse(&self) -> Self {
        let mut inv = vec![0; self.size()];
        for (i, &j) in self.mapping.iter().enumerate() {
            inv[j] = i;
        }
        Self::new(inv)
    }
}

impl Mul for Permutation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut mapping = vec![0; self.size()];
        for i in 0..self.size() {
            mapping[i] = self.apply(rhs.apply(i));
        }
        Self::new(mapping)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use nalgebra::matrix;

    use super::{Operations, Permutation, Translation};
    use crate::base::lattice::Lattice;

    #[test]
    fn test_cartesian_rotations() {
        let lattice = Lattice::new(matrix![
            1.0, -0.5, 0.0;
            0.0, f64::sqrt(3.0) / 2.0, 0.0;
            0.0, 0.0, 1.0;
        ]);
        let rotations = vec![matrix![
            0, -1, 0;
            1, -1, 0;
            0, 0, 1;
        ]];
        let translations = vec![Translation::zeros()];

        let operations = Operations::new(lattice, rotations, translations);

        let actual = operations.cartesian_rotations()[0];
        let expect = matrix![
            -0.5, -f64::sqrt(3.0) / 2.0, 0.0;
            f64::sqrt(3.0) / 2.0, -0.5, 0.0;
            0.0, 0.0, 1.0;
        ];
        assert_relative_eq!(actual, expect);
        assert_eq!(operations.num_operations(), 1)
    }

    #[test]
    fn test_permutation() {
        let permutation = Permutation::new(vec![1, 2, 0]);
        assert_eq!(permutation.apply(0), 1);
        assert_eq!(permutation.inverse(), Permutation::new(vec![2, 0, 1]));
        assert_eq!(
            permutation.clone() * permutation.inverse(),
            Permutation::identity(3)
        );
    }
}
