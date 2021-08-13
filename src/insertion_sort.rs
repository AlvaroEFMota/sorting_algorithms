use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
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
fn insertion_sort_test() {
    let mut things_even = vec![4, 2, 3, 1, 8, 6, 7, 5];
    let mut things_odd = vec![4, 3, 2, 1, 5, 9, 7, 8, 6];
    //super::sort::<_,InsertionSort>(&mut things_even);
    //super::sort::<_,InsertionSort>(&mut things_odd);
    InsertionSort::sort(&mut things_even);
    InsertionSort::sort(&mut things_odd);
    assert_eq!(things_even, &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(things_odd, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
}