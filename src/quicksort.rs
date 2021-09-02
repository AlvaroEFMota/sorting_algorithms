use super::Sorter;

pub struct Quicksort;

impl Sorter for Quicksort {
    fn sort<T>(slice: &mut [T])
    where T: Ord {
        quicksort(slice);
    }
}

fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.len() > 1 {
        let pivot: usize = 0;
        let mut i: usize = 1;
        for j in 1..slice.len() {
            if slice[j] <= slice[pivot] {
                slice.swap(j, i);
                i += 1;
            }
        }
        slice.swap(i-1, pivot);
        quicksort(&mut slice[..i-1]);
        quicksort(&mut slice[i..]);
    }
}


#[test]
fn quicksort_test() {
    let mut things_even = vec![4, 2, 3, 1, 8, 6, 7, 5];
    let mut things_odd = vec![4, 3, 2, 1, 5, 9, 7, 8, 6];
    Quicksort::sort(&mut things_even);
    Quicksort::sort(&mut things_odd);
    assert_eq!(things_even, &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(things_odd, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
}