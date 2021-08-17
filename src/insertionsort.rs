use super::Sorter;

pub struct Insertionsort;

impl Sorter for Insertionsort {
    fn sort<T>(slice: &mut [T])
        where
            T: Ord {
                for unsorted in 1..slice.len() {
                    for i in (1..unsorted+1).rev() {
                        if slice[i] < slice[i-1] {
                            slice.swap(i, i-1);
                        }
                    }
                }
            }
}

#[test]
fn insertionsort_test() {
    let mut things_even = vec![4, 2, 3, 1, 8, 6, 7, 5];
    let mut things_odd = vec![4, 3, 2, 1, 5, 9, 7, 8, 6];
    Insertionsort::sort(&mut things_even);
    Insertionsort::sort(&mut things_odd);
    assert_eq!(things_even, &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(things_odd, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
}