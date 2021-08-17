use super::Sorter;

struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(slice: &mut [T])
    where T: Ord {
        for j in 0..slice.len()-1 {
            let mut smallest_index = j;
            for i in j+1..slice.len() {
                if slice[i] < slice[smallest_index] {
                    smallest_index = i;
                }
            }
            if j != smallest_index {
                slice.swap(j, smallest_index);
            }
        }

    }
}

#[test]
fn selectionsort_test() {
    let mut things_even = vec![4, 2, 3, 1, 8, 6, 7, 5];
    let mut things_odd = vec![4, 3, 2, 1, 5, 9, 7, 8, 6];
    SelectionSort::sort(&mut things_even);
    SelectionSort::sort(&mut things_odd);
    assert_eq!(things_even, &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(things_odd, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
}