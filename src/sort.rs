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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_ok() {
        let mut v = vec![5, 3, 4, 1, 2];
        bubble(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_ok() {
        let mut v = vec![5, 3, 4, 1, 2];
        selection(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }
}
