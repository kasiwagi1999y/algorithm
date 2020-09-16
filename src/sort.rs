pub fn bubble(a: &mut [i32]) {
    for i in 0..a.len() - 1 {
        for j in 0..a.len() - 1 - i {
            if a[j] > a[j + 1] {
            a.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_bubble() {
        let mut a = [3, 1, 4, 1, 5, 9, 2];
        bubble(&mut a);
        assert!(a.windows(2).all(|b| b[0] <= b[1]));
    }
}
