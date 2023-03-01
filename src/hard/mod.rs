mod graph;
mod prime;
mod cons;
mod integer_partitions;

use std::cmp::{min, Ordering};
use std::collections::{HashMap, HashSet};
use crate::hard::prime::mr_test;

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
        if mr_test(i) {
            prime_list.push(i);
            while n % i == 0 && n >= i {
                factors_list.push(i);
                n /= i;
            }
        }
    }
    factors_list
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == b {
        a
    } else if a > b {
        gcd(a - b, b)
    } else {
        gcd(a, b - a)
    }
}

fn smallest_possible_sum_gcd(arr: &[u64]) -> u128 {
    arr.iter().fold(arr[0], |a, b| gcd(a, *b)) as u128 * arr.len() as u128
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

fn mix(s1: &str, s2: &str) -> String {
    let mut s1_hs = HashMap::new();
    let mut s2_hs = HashMap::new();

    s1.chars().filter(|c| c.is_lowercase()).for_each(|c| { s1_hs.entry(c).and_modify(|v| *v += 1).or_insert(1); });
    s2.chars().filter(|c| c.is_lowercase()).for_each(|c| { s2_hs.entry(c).and_modify(|v| *v += 1).or_insert(1); });

    let mut s1: HashMap<char, i32> = s1_hs.into_iter().filter(|(_, n)|  *n > 1).collect();
    let mut s2: HashMap<char, i32> = s2_hs.into_iter().filter(|(_, n)|  *n > 1).collect();

    let mut v = vec![];
    let intersection: Vec<char> = s1.iter().filter(|(c, _)| s2.contains_key(c)).map(|(c, _)| *c).collect();
    for c in intersection  {
        let n1 = *s1.get(&c).unwrap();
        let n2 = *s2.get(&c).unwrap();
        if  n2 > n1 {
            v.push((2, c, n2));
        } else if n2 < n1 {
            v.push((1, c, n1));
        } else {
            v.push((3, c, n1));
        }
        s2.remove(&c);
        s1.remove(&c);
    }

    v.append(&mut s1.into_iter().map(|(c, n)| (1,c, n)).collect());
    v.append(&mut s2.into_iter().map(|(c, n)| (2,c, n)).collect());

    v.sort_by(|(g1, c1, n1), (g2,c2, n2)| {
        if *n1 > *n2 {
            Ordering::Greater
        } else if *n1 < *n2 {
            Ordering::Less
        } else {
            match g2.cmp(g1) {
                Ordering::Less => {Ordering::Less}
                Ordering::Equal => {c2.cmp(c1)}
                Ordering::Greater => {Ordering::Greater}
            }
        }
    });

    v.reverse();

    v.into_iter().map(|(s, c, n)| {
        if s == 1 {
            format!("1:{}", c.to_string().repeat(n as usize))
        } else if s == 2 {
            format!("2:{}", c.to_string().repeat(n as usize))
        } else if s == 3 {
            format!("=:{}", c.to_string().repeat(n as usize))
        } else {
            String::new()
        }
    }).collect::<Vec<String>>().join("/")
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn basics_mix() {
        basics_mix_testing("Are they here", "yes, they are here",
                "2:eeeee/2:yy/=:hh/=:rr");
        basics_mix_testing("looping is fun but dangerous", "less dangerous than coding",
                "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
        basics_mix_testing(" In many languages", " there's a pair of functions",
                "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
        basics_mix_testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
        basics_mix_testing("codewars", "codewars", "");
        basics_mix_testing("A generation must confront the looming ", "codewarrs",
                "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");
    }

    fn basics_mix_testing(s1: &str, s2: &str, exp: &str) -> () {
        assert_eq!(&mix(s1, s2), exp)
    }

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

        let func = smallest_possible_sum;
        assert_eq!(func(&[1, 21, 55]), 3);
        assert_eq!(func(&[3, 13, 23, 7, 83]), 5);
        assert_eq!(func(&[4, 16, 24]), 12);
        assert_eq!(func(&[30, 12]), 12);
        assert_eq!(func(&[60, 12, 96, 48, 60, 24, 72, 36, 72, 72, 48]), 132);
        assert_eq!(func(&[71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71]), 923);
        assert_eq!(func(&[11, 22]), 22);
        assert_eq!(func(&[9]), 9);
        assert_eq!(func(&[1]), 1);
        assert_eq!(func(&[9, 9]), 18);

        let func = smallest_possible_sum_gcd;
        assert_eq!(func(&[17, 527, 323]), 51);
        assert_eq!(func(&[4, 4, 4]), 12);
        assert_eq!(func(&[3570, 7140]), 7140);
        assert_eq!(func(&[12, 12]), 24);
        assert_eq!(func(&[11, 11 * 2, 11 * 5, 11 * 2 * 5, 19 * 11, 19 * 11 * 5, 11 * 19 * 31, 11 * 19 * 31, 11]), 99);
        assert_eq!(func(&[7*17*37*43*128591, 7*17*37*128591, 17*37*43*128591, 17*37*43]), 17*37*4);
        assert_eq!(func(&[7 * 128591, 17 * 128591]), 128591 * 2);
        assert_eq!(func(&[
            2 * 2 * 2 * 2 * 3 * 7 * 7 * 19 * 23 * 23 * 43,
            2 * 3 * 7 * 7 * 19 * 23 * 23 * 43
        ]), 2 * 3 * 7 * 7 * 19 * 23 * 23 * 43 * 2);
        assert_eq!(func(&[
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