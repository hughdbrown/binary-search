use std::cmp::{
    PartialOrd,
};

pub enum SearchEnum {
    Found(usize, usize),
    NotFound(usize),
}

pub fn search<T>(values: &[T], val: T) -> SearchEnum
    where T: Eq + PartialOrd + Copy
{
    let mut lo: usize = 0;
    let mut hi: usize = values.len() - 1;
    let mut iterations: usize = 1;

    while lo <= hi {
        let mid = lo + ((hi - lo) >> 1);
        let test_elem: T = values[mid];
        if test_elem == val {
            return SearchEnum::Found(mid, iterations);
        }
        else if test_elem < val {
            lo = mid + 1;
        }
        else {
            hi = mid - 1;
        }
        iterations += 1;
    }
    SearchEnum::NotFound(iterations)
}
