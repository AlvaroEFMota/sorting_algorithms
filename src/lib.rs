pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Clone + Copy;
}

mod bubblesort;
mod insertionsort;
mod selectionsort;
mod quicksort;
mod mergesort;

#[cfg(test)]
mod tests {
    use super::*; 
    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord {
                slice.sort();
            }
    }



    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1, 7, 6, 8, 5];
        StdSorter::sort(&mut things); // Setting <_,StdSorter> to the function sort, and passing the vector
        assert_eq!(things, &[1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
