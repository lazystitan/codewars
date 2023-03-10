use std::arch::asm;
use std::cmp::{min, Ordering};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

use crate::hard::prime::mr_test;

mod graph;
mod prime;
mod cons;
mod integer_partitions;
mod evaluate;
mod rect_area;
mod spiral;
mod alphabetic_anagrams;

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
    let mut all = (b'a'..=b'z').map(|c| {
        let c = c as char;
        let ct1 = s1.chars().filter(|&lc| lc == c).collect::<String>();
        let ct2 = s2.chars().filter(|&lc| lc == c).collect::<String>();

        match ct1.cmp(&ct2) {
            Ordering::Less => { format!("2:{}", ct2) }
            Ordering::Equal => { format!("=:{}", ct2) }
            Ordering::Greater => { format!("1:{}", ct1) }
        }
    }).filter(|s| s.len() > 3).collect::<Vec<_>>();

    all.as_mut_slice().sort_unstable_by(|s1, s2| {
        match s2.len().cmp(&s1.len()) {
            Ordering::Equal => { s1.cmp(s2) }
            p => p
        }
    });

    all.join("/")
}

fn is_sorted(x: &[char]) -> bool {
    if x.len() <= 1 {
        return true;
    }
    let mut max = x[0];
    for i in 1..x.len() {
        if max > x[i] {
            return false;
        } else {
            max = x[i];
        }
    }
    true
}

fn next_smaller_number(n: u64) -> Option<u64> {
    let mut nv = n.to_string().chars().collect::<Vec<_>>();
    if is_sorted(&nv) {
        return None;
    }
    let last_i = nv.len() - 1;
    let mut len_of_sorted = nv.len() - 2;
    let mut exchange_p = 0;

    for start_i in (0..=len_of_sorted).rev() {
        if !is_sorted(&nv[start_i..=last_i]) {
            exchange_p = start_i;
            break;
        }
    }

    let mut exchange_q = usize::MAX;
    let mut next_smaller_c = '/';
    for i in (exchange_p + 1)..=last_i {
        if nv[i] <  nv[exchange_p] && nv[i] > next_smaller_c && !(exchange_p == 0 && nv[i] == '0') {
            next_smaller_c = nv[i];
            exchange_q = i;
        }
    }

    if exchange_q == usize::MAX {
        return None;
    }

    let m = nv[exchange_p];
    nv[exchange_p] = nv[exchange_q];
    nv[exchange_q] = m;

    nv[(exchange_p + 1) ..= last_i].sort();
    nv[(exchange_p + 1) ..= last_i].reverse();

    Some(nv.into_iter().collect::<String>().parse().unwrap())
}

//1 * 2 + 1 = 3
//1 * 3 + 1 = 4
//3 * 2 + 1 = 7
//4 * 2 + 1 = 9
//3 * 3 + 1 = 10
//4 * 3 + 1 = 13
fn dbl_linear(n: u32) -> u32{
    let n = n as usize;
    let mut x = 0; let mut y = 0;
    let mut u = vec![1];
    (0..=n).for_each(|_| {
        let next_x = 2 * u[x] + 1;
        let next_y = 3 * u[y] + 1;
        match next_x.cmp(&next_y) {
            Ordering::Less => {  u.push(next_x); x += 1; }
            Ordering::Equal => { u.push(next_x); x += 1; y += 1; }
            Ordering::Greater => { u.push(next_y); y += 1;}
        }
    });
    u[n]
}

fn n_linear(m: &[u32], n: usize) -> u32 {
    let mut u = vec![1];
    let mut m_index = vec![0; m.len()];
    let mut r_index = vec![0; m.len()];
    (0..=n).for_each(|_| {
        for (i, &mult) in m.iter().enumerate() {
            r_index[i] = u[m_index[i]] * mult + 1;
        }
        let mut min_r = *r_index.iter().min().unwrap();
        r_index.iter_mut().enumerate().for_each(|(i, v)| {
            if *v == min_r {
                m_index[i] += 1;
            }
        });
        u.push(min_r);
    });

    u[n]
}

#[cfg(test)]
mod test {
    use super::*;

    fn dbl_linear_testing(n: u32, exp: u32) -> () {
        assert_eq!(dbl_linear(n), exp)
    }

    #[test]
    fn pair_test() {
        assert_eq!(n_linear(&[2, 3], 10), 22);
        assert_eq!(n_linear(&[3, 2], 10), 22);
    }

    #[test]
    fn triplet_test() {
        assert_eq!(n_linear(&[5, 7, 8], 10), 64);
        assert_eq!(n_linear(&[5, 7, 8], 11), 65);
    }

    #[test]
    fn basics_dbl_linear() {
        dbl_linear_testing(10, 22);
        dbl_linear_testing(20, 57);
        dbl_linear_testing(30, 91);
        dbl_linear_testing(50, 175);
        dbl_linear_testing(100, 447);
    }

    #[test]
    fn next_smaller_number_test() {
        assert_eq!(Some(12), next_smaller_number(21));
        assert_eq!(Some(790), next_smaller_number(907));
        assert_eq!(Some(513), next_smaller_number(531));
        assert_eq!(None, next_smaller_number(1027));
        assert_eq!(Some(414), next_smaller_number(441));
        assert_eq!(Some(56149891544), next_smaller_number(56149894145));
    }


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
        assert_eq!(func(&[7 * 17 * 37 * 43 * 128591, 7 * 17 * 37 * 128591, 17 * 37 * 43 * 128591, 17 * 37 * 43]), 17 * 37 * 4);
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