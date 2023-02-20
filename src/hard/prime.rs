fn qmul(a: u64, b: u64, m: u64) -> u64 {
    ((a % m) * (b % m)) % m
}


fn qpow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1;
    while exp > 0 {
        //如果指数当前位不为0，累乘到r上
        if exp & 1 > 0 {
            r = qmul(r, base, m);
        }
        //a^(2^n) = a^(2^(n-1)*2) = (a^(2^(n-1)))^2 = a^(2^(n-1))*a^(2^(n-1))
        base = qmul(base, base, m);
        exp >>= 1;
    }
    r
}

pub fn mr_test(n: u64) -> bool {
    if n < 3 || n % 2 == 0 {
        return n == 2;
    }

    //找到形如u * 2^t中的u和t
    let mut u = n - 1;
    let mut t = 0;
    while u % 2 == 0 {
        u /= 2;
        t += 1;
    }

    let ud: [u64; 7] = [2,325,9375,28178,450775,9780504,1795265022];

    for a in ud {
        let mut v = qpow(a, u, n);
        if v == 1 || v == n - 1 || v == 0 {
            continue;
        }

        for j in 1..=t {
            v = qmul(v, v, n);
            if v == n - 1 && j != t {
                v = 1;
                break;
            }
            if v == 1 {
                return  false;
            }
        }
        if v != 1 {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mr_test_test() {
        let primes = [112459, 112481, 112501, 112507, 112543, 112559, 112571, 112573,
            112577, 112583, 112589, 112601, 112603, 112621, 112643, 112657, 112663, 112687, 112691,
            112741];
        // let primes = [7];

        for p in primes {
            assert!(mr_test(p), "p: {}", p);
        }
    }

    #[test]
    fn qmul_test() {
        assert_eq!(qmul(10, 20, 3), 2);
    }

    #[test]
    fn qpow_test() {
        //13 - 1101
        //2^3 + 2^2 + 2^1 * 0 + 2^0
        //3^8 * 3*4 * 3

        assert_eq!(qpow(3, 13, 7), 3);
    }
}