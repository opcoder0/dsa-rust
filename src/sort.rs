pub fn bubble<T: Copy + Ord + PartialOrd>(vec: &mut Vec<T>) {
    let mut sorted = false;
    let mut unsorted_until = vec.len() - 1;
    while !sorted {
        sorted = true;
        for i in 0..unsorted_until {
            if vec[i] > vec[i + 1] {
                (vec[i], vec[i + 1]) = (vec[i + 1], vec[i]);
                sorted = false;
            }
        }
        unsorted_until -= 1;
    }
}

pub fn selection<T: Copy + Ord + PartialOrd>(vec: &mut Vec<T>) {
    let mut smallest = 0;
    let mut start = 0;
    while start != vec.len() - 1 {
        for i in start..vec.len() {
            if vec[i] < vec[smallest] {
                smallest = i;
            }
        }
        if start != smallest {
            (vec[start], vec[smallest]) = (vec[smallest], vec[start]);
        }
        start += 1;
    }
}

pub fn insertion<T: Copy + Ord + PartialOrd>(vec: &mut Vec<T>) {
    let mut gap: usize;
    let mut gap_v: T;

    for i in 1..vec.len() {
        gap = i;
        gap_v = vec[gap];
        let mut j = gap - 1;
        while vec[j] > gap_v {
            vec[gap] = vec[j];
            gap = j;
            if j == 0 {
                break;
            } else {
                j -= 1;
            }
        }
        vec[gap] = gap_v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_ok() {
        let mut v = vec![5, 3, 4, 1, 2];
        bubble(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn selection_sort_ok() {
        let mut v = vec![5, 3, 4, 1, 2];
        selection(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn insertion_sort_ok() {
        let mut v = vec![5, 3, 4, 1, 2];
        insertion(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }
}
