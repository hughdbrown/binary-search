pub enum SearchEnum {
    Found(usize, usize),
    NotFound(usize),
}

pub fn search<T>(values: &[T], val: &T) -> SearchEnum
    where T: Eq + Clone + Into<usize>
{
    let mut iterations: usize = 0;
    if values.len() > 0 {
        let mut lo: usize = 0;
        let mut hi: usize = values.len() - 1;

        let valu: usize = Into::<usize>::into(val.clone());
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            let test_elem: T = values[mid].clone();
            let key: usize = Into::<usize>::into(test_elem);

            iterations += 1;
            if key == valu {
                return SearchEnum::Found(mid, iterations);
            }
            else if key < valu {
                lo = mid + 1;
            }
            else {
                if mid == 0 { break; }
                hi = mid - 1;
            }
        }
    }
    SearchEnum::NotFound(iterations)
}
