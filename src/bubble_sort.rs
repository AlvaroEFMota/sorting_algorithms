use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
        where
            T: Ord {
                let mut swapped = true;
                while swapped {
                    swapped = false;
                    for i in 0..(slice.len()-1) {
                        if slice[i] > slice[i+1] {
                            slice.swap(i, i+1);
                            swapped = true;
                        }
                    }
                }
            }
}

#[test]
fn bubble_sort_works() {
    let mut things_even = vec![4, 2, 3, 1, 8, 6, 7, 5];
    let mut things_odd = vec![4, 3, 2, 1, 5, 9, 7, 8, 6];
    //super::sort::<_,BubbleSort>(&mut things_even);
    //super::sort::<_,BubbleSort>(&mut things_odd);
    BubbleSort::sort(&mut things_even);
    BubbleSort::sort(&mut things_odd);

    assert_eq!(things_even, &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(things_odd, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
}