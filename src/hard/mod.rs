mod prime;

use std::cmp::min;
use std::collections::{HashMap, HashSet};

//5+(6-2)*9+3^(7-1)
//562-9*+371-
//+^
//1^2^3+4
//123^^4+
//244/6/2*/5109/92*^^^0-54/+/1*4-1-28/1^+10*-93^+63/+8-
//244/6/2*/51099^/2*^^0-54/+/1*4-1-281^/+10*-93^+63/+8-
//1+2^(3*2/0)
//123
//+^(*
fn to_postfix(infix: &str) -> String {
    // "".to_string()
    let mut s = vec![];
    let mut r = vec![];
    for c in infix.chars() {
        if c.is_numeric() {
            r.push(c);
        } else {
            if c == '(' {
                s.push(c);
            } else if c == ')' {
                while *(s.last().unwrap()) != '(' {
                    r.push(s.pop().unwrap());
                }
                s.pop();
            } else if ['+', '-'].contains(&c) {
                while !s.is_empty()
                    && ['*', '/', '+', '-', '^'].contains(s.last().unwrap()) {
                    r.push(s.pop().unwrap());
                }
                s.push(c);
            } else if ['*', '/'].contains(&c) {
                while !s.is_empty()
                    && ['*', '/', '^'].contains(s.last().unwrap()) {
                    r.push(s.pop().unwrap());
                }
                s.push(c);
            } else if c == '^' {
                // while !s.is_empty()
                //     && *(s.last().unwrap()) != '^' {
                //     r.push(s.pop().unwrap());
                // }
                s.push(c);
            } else {
                continue;
            }
        }
    }

    while !s.is_empty() {
        r.push(s.pop().unwrap());
    }

    r.into_iter().collect()
}

fn is_prime(n: u64) -> bool {
    let upper = ((n as f64).sqrt()) as u64;
    for i in 2..=upper {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn prime_factorisation(mut n: u64, prime_list: &mut Vec<u64>) -> Vec<u64> {
    let mut factors_list = vec![1];

    //prime already known
    for &prime in prime_list.iter() {
        while n % prime == 0 && n >= prime {
            factors_list.push(prime);
            n /= prime;
        }

        if n < prime {
            return factors_list;
        }
    }

    //find new if needed
    for i in (prime_list[prime_list.len() - 1] + 1)..=n {
        if is_prime(i) {
            prime_list.push(i);
            while n % i == 0 && n >= i {
                factors_list.push(i);
                n /= i;
            }
        }
    }
    factors_list
}

fn smallest_possible_sum(arr: &[u64]) -> u128 {
    if arr.len() == 0 {
        return 0;
    }
    let len = arr.len();
    let mut hs = vec![];
    let mut prime_list = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];

    arr.iter().for_each(|&x| hs.push(prime_factorisation(x, &mut prime_list)));

    let mut sets: Vec<HashSet<_>> = hs.iter().map(|x|
        HashSet::from_iter(x.iter())
    ).collect();

    let mut last = sets.pop().unwrap();
    for s in sets {
        last = s.intersection(&last).map(|n| *n).collect();
    }

    let mut dividers_count = HashMap::new();

    hs.iter().for_each(|x| last.iter().for_each(|&&n| {
        let count = x.iter().filter(|&&x| x == n).count();
        dividers_count.entry(n)
            .and_modify(|x| *x = min(*x, count))
            .or_insert(count);
    }));


    let mut biggest_factor = 1;
    for &n in last {
        let count = dividers_count.get(&n).unwrap();
        for _ in 0..*count {
            biggest_factor *= n as u128;
        }
    }

    biggest_factor * len as u128
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_postfix_do_test(actual: &str, expected: &str) {
        assert_eq!(actual, expected, "\nYour answer (left) is not the correct answer (right)")
    }

    #[test]
    fn to_postfix_tests() {
        to_postfix_do_test(&to_postfix("2+7*5"), "275*+");
        to_postfix_do_test(&to_postfix("3*3/(7+1)"), "33*71+/");
        to_postfix_do_test(&to_postfix("5+(6-2)*9+3^(7-1)"), "562-9*+371-^+");
        to_postfix_do_test(&to_postfix("(5-4-1)+9/5/2-7/1/7"), "54-1-95/2/+71/7/-");
        to_postfix_do_test(&to_postfix("1^2^3"), "123^^");
    }

    #[test]
    fn char_string_test() {
        let s = String::from("1");
        for c in s.chars() {
            assert_eq!(c as u32, '1' as u32);
            assert_eq!(c, '1');
        }
    }

    #[test]
    fn smallest_possible_sum_tests() {
        assert_eq!(smallest_possible_sum(&[1, 21, 55]), 3);
        assert_eq!(smallest_possible_sum(&[3, 13, 23, 7, 83]), 5);
        assert_eq!(smallest_possible_sum(&[4, 16, 24]), 12);
        assert_eq!(smallest_possible_sum(&[30, 12]), 12);
        assert_eq!(smallest_possible_sum(&[60, 12, 96, 48, 60, 24, 72, 36, 72, 72, 48]), 132);
        assert_eq!(smallest_possible_sum(&[71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71]), 923);
        assert_eq!(smallest_possible_sum(&[11, 22]), 22);
        assert_eq!(smallest_possible_sum(&[9]), 9);
        assert_eq!(smallest_possible_sum(&[1]), 1);
        assert_eq!(smallest_possible_sum(&[9, 9]), 18);


        assert_eq!(smallest_possible_sum(&[17, 527, 323]), 51);
        assert_eq!(smallest_possible_sum(&[4, 4, 4]), 12);
        assert_eq!(smallest_possible_sum(&[3570, 7140]), 7140);
        assert_eq!(smallest_possible_sum(&[12, 12]), 24);
        assert_eq!(smallest_possible_sum(&[11, 11 * 2, 11 * 5, 11 * 2 * 5, 19 * 11, 19 * 11 * 5, 11 * 19 * 31, 11 * 19 * 31, 11]), 99);
        // assert_eq!(smallest_possible_sum(&[7*17*37*43*128591, 7*17*37*128591, 17*37*43*128591, 17*37*43]), 17*37*4);
        assert_eq!(smallest_possible_sum(&[7 * 128591, 17 * 128591]), 128591 * 2);
        assert_eq!(smallest_possible_sum(&[
            2 * 2 * 2 * 2 * 3 * 7 * 7 * 19 * 23 * 23 * 43,
            2 * 3 * 7 * 7 * 19 * 23 * 23 * 43
        ]), 2 * 3 * 7 * 7 * 19 * 23 * 23 * 43 * 2);
        assert_eq!(smallest_possible_sum(&[
            2 * 2 * 3 * 13 * 37,
            2 * 2 * 3 * 13 * 37,
            2 * 2 * 3 * 13 * 37 * 19,
            2 * 2 * 3 * 13 * 37 * 13,
            2 * 2 * 3 * 13 * 37 * 7,
            2 * 2 * 3 * 13 * 37 * 3,
            2 * 2 * 3 * 13 * 37 * 2
        ]), 40404);
    }

    #[test]
    fn is_prime_test() {
        assert!(is_prime(128591));
        assert!(is_prime(17));
        assert!(is_prime(37));
        assert!(is_prime(43));
        assert!(!is_prime(7 * 17 * 37 * 128591));
    }
}