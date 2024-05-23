use bitvector::BitVector;

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

pub fn adaptive_nonmax_suppression<T>(vec: &mut [T], n: usize) -> Vec<T>
where
    T: Matchable,
    T: Copy,
{
    assert!(n <= vec.len());

    let mut maximal_keypoints: Vec<T> = vec![];
    for i in 1..vec.len() - 1 {
        let d1 = &vec[i];
        let mut min_dist: usize = usize::MAX;
        let mut min_idx: usize = 0;

        for (j, e) in vec.iter().enumerate().take(i) {
            let dist = e.distance(d1);
            if dist < min_dist {
                min_dist = dist;
                min_idx = j;
            }
        }

        vec.swap(i, min_idx);
    }

    for e in vec.iter().take(n) {
        maximal_keypoints.push(*e);
    }

    maximal_keypoints
}
