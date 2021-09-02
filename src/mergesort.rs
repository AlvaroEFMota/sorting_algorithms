use super::Sorter;

struct Mergesort;

impl Sorter for Mergesort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Clone + Copy,
    {
        let size = slice.len();
        merge_sort(slice, 0, size);
    }
}

fn merge_sort<T>(v: &mut [T], start: usize, end: usize)
where
    T: Ord + Clone + Copy,
{
    if end - start > 1 {
        let middle = (start + end) / 2;
        merge_sort(v, start, middle);
        merge_sort(v, middle, end);

        let mut aux_j: Vec<T> = Vec::with_capacity(middle - start);
        let mut aux_k: Vec<T> = Vec::with_capacity(end - middle);

        for i in start..middle {
            aux_j.push(v[i].clone());
        }
        for i in middle..end {
            aux_k.push(v[i].clone());
        }

        let mut j = 0;
        let mut k = 0;
        let mut index_i: usize = 0;
        for i in start..end {
            // Aprender a colocar condições no for
            if j >= middle - start || k >= end - middle {
                break;
            }

            if aux_j[j] < aux_k[k] {
                v[i] = aux_j[j];
                j += 1;
            } else {
                v[i] = aux_k[k];
                k += 1;
            }
            index_i = i;
        }
        index_i += 1;

        for p in j..(middle - start) {
            v[index_i] = aux_j[p];
            index_i += 1;
        }

        for p in k..(end - middle) {
            v[index_i] = aux_k[p];
            index_i += 1;
        }
    }
}

#[test]
fn mergesort_test() {
    let mut things_even = vec![4, 2, 3, 1, 8, 6, 7, 5];
    let mut things_odd = vec![4, 3, 2, 1, 5, 9, 7, 8, 6];
    Mergesort::sort(&mut things_even);
    Mergesort::sort(&mut things_odd);
    assert_eq!(things_even, &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(things_odd, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
