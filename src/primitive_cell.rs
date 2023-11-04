use nalgebra::Vector3;
use std::collections::HashMap;

use crate::cell::{Cell, Position};
use crate::error::MoyoError;
use crate::operation::{CellWithOperations, Permutation, Rotation, Translation};
use crate::transformation::Transformation;

/// Return primitive cell and transformation matrix from the primitive cell to the input cell
pub fn search_primitive_cell(
    cell: &Cell,
    symprec: f64,
) -> Result<(CellWithOperations, Transformation), MoyoError> {
    let (reduced_lattice, reduced_trans) = cell.lattice.minkowski_reduce()?;
    let reduced_cell = cell.transform(&reduced_trans);

    // Check if symprec is sufficiently small
    let minimum_basis_norm = reduced_lattice
        .basis
        .column_iter()
        .map(|v| v.norm())
        .fold(f64::INFINITY, f64::min);
    let rough_symprec = 2.0 * symprec;
    if rough_symprec < minimum_basis_norm / 2.0 {
        return Err(MoyoError::TooSmallSymprecError);
    }

    // Choose atomic specie with the smallest occurrence
    let mut counter = HashMap::new();
    for number in reduced_cell.numbers.iter() {
        let count = counter.entry(number).or_insert(0);
        *count += 1;
    }
    let pivot_atomic_specie = *counter.iter().min_by_key(|(_, count)| *count).unwrap().0;
    let pivot_site_indices = reduced_cell
        .numbers
        .iter()
        .enumerate()
        .filter(|(_, number)| *number == pivot_atomic_specie)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    // Try possible translations: overlap the `src`the site to the `dst`th site
    // TODO: this part takes O(num_atoms^3)
    let mut permutations_tmp = vec![];
    let src = pivot_site_indices[0];
    for dst in pivot_site_indices.iter() {
        let translation = reduced_cell.positions[*dst] - reduced_cell.positions[src];
        let new_positions: Vec<Position> = reduced_cell
            .positions
            .iter()
            .map(|pos| pos + translation)
            .collect();

        // Because the translation may not be optimal to minimize distance between input and acted positions,
        // use a larger symprec (diameter of a Ball) for finding correspondence
        if let Some(permutation) =
            solve_correspondence(&reduced_cell, &new_positions, rough_symprec)
        {
            permutations_tmp.push(permutation);
        }
    }
    assert!(permutations_tmp.len() > 0);

    // Purify translations by permutations
    let mut translations_and_permutations = vec![];
    for permutation in permutations_tmp.iter() {
        let (translation, distance) = symmetrize_translation_from_permutation(
            &reduced_cell,
            permutation,
            &Rotation::identity(),
        );
        if distance < symprec {
            translations_and_permutations.push((translation, permutation));
        }
    }
    assert!(translations_and_permutations.len() > 0);

    if reduced_cell.num_atoms % translations_and_permutations.len() != 0 {
        return Err(MoyoError::PrimitiveCellSearchError);
    }

    unimplemented!();
}

/// Return correspondence between the input and acted positions.
/// Assume that reduced_cell is Minkowski reduced and symprec is sufficiently small for Babai's algorithm.
/// Search permutation such that new_positions[i] = reduced_cell.positions[permutation[i]].
/// Then, a corresponding symmetry operation moves the i-th site into the permutation[i]-th site.
/// Be careful that the current implementation takes O(num_atoms^2) time!
fn solve_correspondence(
    reduced_cell: &Cell,
    new_positions: &Vec<Position>,
    symprec: f64,
) -> Option<Permutation> {
    let num_atoms = reduced_cell.num_atoms;

    let mut permutation = vec![0; num_atoms];
    let mut visited = vec![false; num_atoms];

    for i in 0..num_atoms {
        for j in 0..num_atoms {
            if visited[j] || reduced_cell.numbers[i] != reduced_cell.numbers[j] {
                continue;
            }

            let mut frac_displacement = new_positions[j] - reduced_cell.positions[i];
            frac_displacement -= frac_displacement.map(|e| e.round());
            let distance = reduced_cell
                .lattice
                .cartesian_coords(&frac_displacement)
                .norm();
            if distance < symprec {
                permutation[i] = j;
                visited[j] = true;
                break;
            }
        }
    }

    if visited.iter().all(|&v| v) {
        Some(permutation)
    } else {
        None
    }
}

fn symmetrize_translation_from_permutation(
    reduced_cell: &Cell,
    permutation: &Permutation,
    rotation: &Rotation,
) -> (Translation, f64) {
    // argmin_{t} sum_{i} | pbc(rotation * positions[i] + t - positions[permutation[i]]) |^2
    //   = 1/num_atoms * sum_{i} pbc(positions[permutation[i]] - rotation * positions[i])
    let num_atoms = reduced_cell.num_atoms;

    let translation = (0..num_atoms)
        .map(|i| {
            let mut frac_displacement = reduced_cell.positions[permutation[i]]
                - rotation.map(|e| e as f64) * reduced_cell.positions[i];
            frac_displacement -= frac_displacement.map(|e| e.round());
            frac_displacement
        })
        .sum::<Vector3<_>>()
        / (num_atoms as f64);
    let new_positions = reduced_cell
        .positions
        .iter()
        .map(|pos| rotation.map(|e| e as f64) * pos + translation)
        .collect::<Vec<_>>();
    let distance = (0..num_atoms)
        .map(|i| (reduced_cell.positions[permutation[i]] - new_positions[i]).norm())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    (translation, distance)
}
