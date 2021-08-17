pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

pub fn sort<T, S>(slice: &mut [T])
where T: Ord, S: Sorter {
    S::sort(slice)
}

mod bubblesort;
mod insertionsort;
mod selectionsort;
mod quicksort;

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
        sort::<_,StdSorter>(&mut things); // Setting <_,StdSorter> to the function sort, and passing the vector
        assert_eq!(things, &[1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
