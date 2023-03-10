fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut spiral = vec![vec![0; size]; size];
    let mut value = 1;

    for j in 0..(size + 1) / 2 {
        for i in j..(size - j) {
            spiral[i][j] = value;
            spiral[j][i] = value;

            spiral[i][size - 1 - j] = value;
            spiral[size - 1 - j][i] = value;
        }

        value = (value + 1) % 2;

        if j < (size - 1) / 2 || spiral[j][j - 1] == 1 {
            spiral[j + 1][j] = value;
        }
    }

    spiral
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test5() {
        assert_eq!(
            spiralize(5),
            [
                [1, 1, 1, 1, 1],
                [0, 0, 0, 0, 1],
                [1, 1, 1, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 1, 1, 1, 1],
            ],
        );
    }

    #[test]
    fn test8() {
        assert_eq!(
            spiralize(8),
            [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ],
        );
    }
}
