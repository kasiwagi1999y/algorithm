pub fn insertion<T: PartialOrd + Copy>(a: &mut [T]) {
    for i in 1..a.len() {
        let t = a[i];
        let mut j = i;
        while 0 < j && t < a[j - 1] {
            a[j] = a[j - 1];
            j -= 1;
        }
        a[j] = t;
    }
}

pub fn bubble<T: PartialOrd>(a: &mut [T]) {
    for i in 0..a.len() - 1 {
        for j in 0..a.len() - 1 - i {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
            }
        }
    }
}

pub fn selection<T: Ord>(a: &mut [T]) {
    for i in 0..a.len() {
        if let Some((x, _)) = a[i..].iter().enumerate().min_by_key(|&(_, x)| x) {
            a.swap(i, i + x);
        }
    }
}

pub fn merge() {}

pub fn quick<T: PartialOrd + Copy>(a: &mut [T]) {
    let mut i = 0;
    let mut j = a.len() - 1;
    let pivot = a[(j - i) / 2];
    loop {
        while a[i] < pivot {
            i += 1;
        }
        while a[j] > pivot {
            j -= 1;
        }
        if i >= j {
            break;
        }
        a.swap(i, j);
        i += 1;
        j -= 1;
    }
    if i > 1 {
        quick(&mut a[..i]);
    }
    if j + 1 < a.len() {
        quick(&mut a[(j + 1)..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_insertion() {
        let mut a = [3, -1, 1, 4, 1, 5, 9, 2];
        insertion(&mut a);
        assert!(a.windows(2).all(|b| b[0] <= b[1]));
    }

    #[test]
    fn it_bubble() {
        let mut a = [6, 5, 3, 5, 8, 9, 7, 9];
        bubble(&mut a);
        assert!(a.windows(2).all(|b| b[0] <= b[1]));
    }

    #[test]
    fn it_selection() {
        let mut a = [3, 2, 3, 8, 4, 6, 2, 6];
        bubble(&mut a);
        assert!(a.windows(2).all(|b| b[0] <= b[1]));
    }
    #[test]
    fn it_merge() {
        let mut a = [4, 3, 3, 8, 3, 2, 7, 9];
    }

    #[test]
    fn it_quick() {
        let mut a = [5, 0, 2, 8, 8, 4, 1, 9];
        quick(&mut a);
        assert!(a.windows(2).all(|b| b[0] <= b[1]));
    }
}
