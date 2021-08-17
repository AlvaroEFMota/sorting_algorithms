use super::Sorter;

pub struct Bubblesort;

impl Sorter for Bubblesort {
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
fn bubblesort_test() {
    let mut things_even = vec![4, 2, 3, 1, 8, 6, 7, 5];
    let mut things_odd = vec![4, 3, 2, 1, 5, 9, 7, 8, 6];
    //super::sort::<_,Bubblesort>(&mut things_even);
    //super::sort::<_,Bubblesort>(&mut things_odd);
    Bubblesort::sort(&mut things_even);
    Bubblesort::sort(&mut things_odd);

    assert_eq!(things_even, &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(things_odd, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
}