fn partition(n: u64, m: u64) -> u64 {
    if n == 1 || m == 1 {
        1
    } else if n == m {
        1 + partition(n, m - 1)
    } else if n < m {
        partition(n, n)
    } else { //if n > m
        partition(n - m, m) + partition(n, m - 1)
    }
}

fn int_part(n: i64, m: i64) -> Vec<Vec<i64>> {
    if n == 1 {
        vec![vec![1]]
    } else if m == 1 {
        vec![vec![1; n as usize]]
    } else if n == m {
        let mut r = vec![vec![m]];
        r.append(&mut int_part(n, m - 1));
        r
    } else if n < m {
        int_part(n, n)
    } else { //if n > m
        let mut t = int_part(n - m, m);
        t.iter_mut().for_each(|x| x.push(m));
        t.append(&mut int_part(n, m - 1));
        t
    }
}

fn part(n: i64) -> String {
    let mut v: Vec<i64> = int_part(n, n)
        .into_iter()
        .map(
            |v| v.into_iter().fold(1, |a, b| a * b)
        )
        .collect();
    v.sort();
    v.dedup();
    let mid = if v.len() % 2 == 0 {
        (v[((v.len() as f64) / 2.0) as usize] as f64 + v[((v.len() - 1) / 2) as usize] as f64) / 2.0
    } else if v.len() == 1 {
        v[0] as f64
    } else {
        v[(v.len() / 2)] as f64
    };
    format!("Range: {} Average: {:.2} Median: {:.2}",
            v.last().unwrap() - v.first().unwrap(),
            v.iter().sum::<i64>() as f64 / v.len() as f64,
            mid
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pt_test() {
        assert_eq!(partition(5, 5), 7);
        assert_eq!(partition(50, 50), 204226);
    }

    fn testequal(ans: &str, sol: &str) {
        assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
    }

    #[test]
    fn returns_expected() {
        testequal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
        testequal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
        testequal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
        testequal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
        testequal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
    }
}