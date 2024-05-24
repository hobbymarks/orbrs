use bitvector::BitVector;
use std::cmp::Ordering;

pub type Point = (i32, i32);
pub type IndexMatch = (usize, usize);

pub trait Matchable {
    fn distance(&self, other: &Self) -> usize;
}

pub fn match_indices<T>(vec_a: &[T], vec_b: &[T]) -> Vec<IndexMatch>
where
    T: Matchable,
{
    assert_eq!(vec_a.len(), vec_b.len());

    let mut index_vec = vec![];
    let len = vec_a.len();
    let mut matched_indices = BitVector::new(len);

    for (ia, ea) in vec_a.iter().enumerate().take(len) {
        let mut min_dist: usize = usize::MAX;
        let mut matched_index: usize = 0;
        for (ib, eb) in vec_b.iter().enumerate().take(len) {
            if matched_indices.contains(ib) {
                continue;
            }

            let dist = ea.distance(eb);
            if dist < min_dist {
                min_dist = dist;
                matched_index = ib;
            }
        }

        index_vec.push((ia, matched_index));
        matched_indices.insert(matched_index);
    }

    index_vec
}

pub fn adaptive_nonmax_suppression<T>(vec_desc: &mut [T], n: usize) -> Vec<T>
where
    T: Matchable,
    T: Copy,
{
    if n == 0 {
        return Vec::new();
    }

    assert!(n <= vec_desc.len());

    let mut suppression_radius = vec![usize::MAX; vec_desc.len()];

    for i in 1..vec_desc.len() {
        for j in 0..i {
            let dist = vec_desc[i].distance(&vec_desc[j]);
            if dist < suppression_radius[i] {
                suppression_radius[i] = dist;
            }
        }
    }

    let mut vec_with_radius: Vec<_> = vec_desc.iter_mut().zip(suppression_radius).collect();
    vec_with_radius.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

    vec_with_radius
        .into_iter()
        .take(n)
        .map(|(kp, _)| *kp)
        .collect()
}
