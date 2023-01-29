use std::cmp::min;

fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    // a.retain(|x| !b.contains(x));
    a.into_iter().filter(|item| {
        !b.contains(item)
    }).collect()
}

fn find_next_square(sq: u64) -> Option<u64> {
    let b = (sq as f64).sqrt() as u64;
    (b * b == sq).then(|| (b + 1).pow(2))
}

fn find_even_index(arr: &[i32]) -> Option<usize> {
    let arr_len = arr.len();
    let mut left = vec![0; arr_len];
    let mut right = vec![0; arr_len];
    for (i, value) in arr.iter().enumerate() {
        if i == 0 {
            left[0] = *value;
        } else {
            left[i] = left[i - 1] + *value;
        }
    }

    for (i, value) in arr.iter().rev().enumerate() {
        if i == 0 {
            right[arr_len - 1] = *value;
        } else {
            right[arr_len - i - 1] = right[arr_len - i] + *value;
        }
    }

    for i in 0..arr_len {
        if left[i] == right[i] {
            return Some(i);
        }
    }
    None
}

fn duplicate_encode(word: &str) -> String {
    /*
    use std::collections::HashMap;
    let mut m = HashMap::new();
    for s in word.chars() {
        let s = s.to_ascii_lowercase();
        if m.contains_key(&s) {
            *m.get_mut(&s).unwrap() += 1;
        } else {
            m.insert(s, 1);
        }
    }

    let mut r = String::new();

    for s in word.chars() {
        let s = s.to_ascii_lowercase();
        if m[&s] > 1 {
            r.push(')');
        } else {
            r.push('(');
        }
    }

    return r;
     */

    let mut enc = std::collections::HashMap::new();
    for c in word.to_lowercase().chars() {
        *enc.entry(c).or_insert(0) += 1;
    }
    word.to_lowercase().chars().map(|c| match *enc.get(&c).unwrap() {
        1 => '(',
        _ => ')'
    }).collect()
}

fn dna_strand(dna: &str) -> String {
    dna.chars().map(|c| match c {
        'A' => 'T',
        'C' => 'G',
        'T' => 'A',
        'G' => 'C',
        _ => ' '
    }).collect()
}

fn validate_pin(pin: &str) -> bool {
    false
}

fn find_short(s: &str) -> u32 {
    s.split_whitespace().map(|s| s.len()).min().unwrap_or(0) as u32
}

fn alphabet_position(text: &str) -> String {
    text
        .to_lowercase().chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| (c as u32 - 'a' as u32 + 1).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn is_valid_walk(walk: &[char]) -> bool {
    let mut p = (0, 0);
    let mut s = 0;
    for c in walk {
        s += 1;
        match c {
            'n' => p.1 += 1,
            's' => p.1 -= 1,
            'e' => p.0 += 1,
            'w' => p.0 -= 1,
            _ => {}
        }
    }

    p == (0, 0) && s == 10
}

fn make_readable(seconds: u32) -> String {
    let mut left = seconds;
    let s = left % 60;
    left = left / 60;
    let m = left % 60;
    left = left / 60;
    let h = left;

    format!("{:0>2}:{:0>2}:{:0>2}", h, m, s)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    /*
    let mut pair;
    let arr_len = arr.len();
    let mut marks = vec![true; arr_len];
    let mut marks_changed = false;
    if arr.len() < 2 { return arr.iter().map(|d| *d).collect() }
    loop {
        let mut i = 0;
        let mut j = 1;
        marks_changed = false;
        while j < arr_len {
            while i < arr_len - 1 && !marks[i] {
                i+= 1;
            }
            if i >= arr_len - 1 {
                break;
            }
            j = i + 1;
            while j < arr_len && !marks[j] {
                j+= 1;
            }
            if j > arr_len - 1 {
                break;
            }

            pair = (arr[i], arr[j]);
            match pair {
                (Direction::North, Direction::South)
                | (Direction::South, Direction::North)
                | (Direction::West, Direction::East)
                | (Direction::East, Direction::West) => {
                    marks_changed = true;
                    marks[i] = false;
                    marks[j] = false;
                    i = j + 1;
                    j = i + 1;
                }
                _ => {
                    i += 1;
                    j += 1;
                }
            }
        }
        if !marks_changed {
            break;
        }
    }

    arr.iter()
        .enumerate()
        .filter(|(i, _)| marks[*i])
        .map(|(_, d)| *d)
        .collect()
     */

    let mut s = vec![];
    for d in arr {
        match (d, s.last()) {
            (Direction::North, Some(Direction::South))
            | (Direction::South, Some(Direction::North))
            | (Direction::West, Some(Direction::East))
            | (Direction::East, Some(Direction::West)) => {
                s.pop();
            }
            _ => s.push(*d)
        }
    }
    s
}

fn to_camel_case(text: &str) -> String {
    /*
    let mut s = String::new();
    let mut flag = false;
    for c in text.chars() {
        if c == '-' || c == '_' {
            flag = true;
        } else if flag {
            s += &c.to_uppercase().to_string();
            flag = false;
        } else {
            s += &c.to_string();
        }

    }
    s
    */
    text.split(&['-', '_'])
        .enumerate()
        .map(|(i, w)| match i {
        0 => w.to_string(),
        _ => w[..1].to_uppercase() + &w[1..]
    }).collect()
}

fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    let mut p = 0;
    loop {
        let left = target - numbers[p];
        for (i, &n) in numbers[(p + 1)..].iter().enumerate() {
            if left == n {
                return (p, i + p + 1);
            }
        }
        p += 1;
    }
}

fn rot13(message: &str) -> String {
    message.chars().map(|c| {
        if c.is_alphabetic() && !c.is_uppercase() {
            let mut c  = c as u32 + 13;
            if  c > 'z' as u32 {
                c = c - 'z' as u32 + 'a' as u32 - 1;
            }
            char::from_u32(c).unwrap()
        } else if c.is_alphabetic() && c.is_uppercase() {
            let mut c  = c as u32 + 13;
            if  c > 'Z' as u32 {
                c = c - 'Z' as u32 + 'A' as u32 - 1;
            }
            char::from_u32(c).unwrap()
        } else {
            c
        }
    }).collect()
    /*
    message.chars().map(|c| {
        match c {
            'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
            'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
            _ => c,
        }
    }).collect()
     */
}

fn tower_builder(n_floors: usize) -> Vec<String> {
    /*
    let mut r = vec![];
    let m = n_floors * 2 - 1;
    for f in 0..n_floors {
        let f = f + 1;
        let mut s = String::new();
        let n = f * 2 - 1;
        let w = (m - n) /2;
        for _ in 0..w {
            s.push(' ');
        }
        for _ in 0..n {
            s.push('*');
        }
        for _ in 0..w {
            s.push(' ');
        }
        r.push(s);
    }
    r
     */
    let mut r = Vec::with_capacity(n_floors);
    for floor in 1..=n_floors {
        r.push(
            format!(
                "{}{}{}",
                " ".repeat(n_floors - floor),
                "*".repeat(2*floor - 1),
                " ".repeat(n_floors - floor)
            )
        );
    }
    r
}

fn parts_sums(ls: &[u64]) -> Vec<u64> {
    let mut r = vec![0; ls.len() + 1];
    for i in (0..ls.len()).rev() {
        r[i] = r[i + 1] + ls[i];
    }
    r
}

fn valid_isbn10(isbn: &str) -> bool {
    /*
    if isbn.len() != 10 ||
        isbn[0..9].chars().filter(|c| !c.is_numeric()).collect::<Vec<char>>().len() != 0 {
        return false;
    }
    isbn.chars()
        .enumerate()
        .map(|(i, c)| {
            if c == 'X' {
                return (i + 1) * 10;
            }
            (i + 1) * (c as u32 - '0' as u32) as usize
        })
        .sum::<usize>() % 11 == 0
     */
    isbn.len() == 10 &&
    isbn.chars().enumerate().all(|(i, c)| c.is_numeric() || (c == 'X' && i == 9)) &&
    isbn.chars().enumerate().map(|(i, c)| c.to_digit(10).unwrap_or(10) * (i as u32 + 1)).sum::<u32>() % 11 == 0
}

/// Write a program that will calculate the number of trailing zeros in a factorial of a given
/// number.
fn zeros(n: u64) -> u64 {
    if n >= 5 {
        return n / 5 + zeros(n / 5);
    } else {
        return 0;
    }
    // std::iter::successors(Some(n/5), |&n| Some(n/5)).take_while(|&n| n > 0).sum()
}

fn decompose(n: i64) -> Option<Vec<i64>> {

}

fn main() {
    assert!(array_diff(vec![1, 2, 2, 2, 3], vec![2]) == vec![1, 3]);
}

#[cfg(test)]
mod tests {
    use super::Direction::*;
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(find_next_square(121), Some(144));
        assert_eq!(find_next_square(625), Some(676));
        assert_eq!(find_next_square(319_225), Some(320_356));
        assert_eq!(find_next_square(15_241_383_936), Some(15_241_630_849));
        assert_eq!(find_next_square(155), None);
        assert_eq!(find_next_square(342_786_627), None);
    }

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(arr: &[i32], expected: Option<usize>) {
        assert_eq!(find_even_index(arr), expected, "{ERR_MSG} with arr = {arr:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 3, 2, 1], Some(3));
        dotest(&[1, 100, 50, -51, 1, 1], Some(1));
        dotest(&[1, 2, 3, 4, 5, 6], None);
        dotest(&[20, 10, 30, 10, 10, 15, 35], Some(3));
        dotest(&[20, 10, -80, 10, 10, 15, 35], Some(0));
        dotest(&[10, -80, 10, 10, 15, 35, 20], Some(6));
        dotest(&(1..100).collect::<Vec<_>>(), None);
        dotest(&[0, 0, 0, 0, 0], Some(0));
        dotest(&[-1, -2, -3, -4, -3, -2, -1], Some(3));
        dotest(&(-100..-1).collect::<Vec<_>>(), None);
        dotest(&[8, 8], None);
        dotest(&[8, 0], Some(0));
        dotest(&[0, 8], Some(1));
        dotest(&[7, 3, -3], Some(0));
        dotest(&[8], Some(0));
        dotest(&[10, -10], None);
        dotest(&[-3, 2, 1, 0], Some(3));
        dotest(&[-15, 5, 11, 17, 19, -17, 20, -6, 17, -17, 19, 16, -15, -6, 20, 17], Some(8));
    }

    #[test]
    fn run_tests() {
        assert_eq!(duplicate_encode("din"), "(((");
        assert_eq!(duplicate_encode("recede"), "()()()");
        assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
        assert_eq!(duplicate_encode("(( @"), "))((");
    }

    fn dna_strand_dotest(s: &str, expected: &str) {
        let actual = dna_strand(s);
        assert!(actual == expected,
                "With dna = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn dna_strand_tests() {
        dna_strand_dotest("AAAA", "TTTT");
        dna_strand_dotest("ATTGC", "TAACG");
        dna_strand_dotest("GTAT", "CATA");
    }

    #[test]
    fn invalid_length_tests() {
        assert_eq!(validate_pin("1"), false);
        assert_eq!(validate_pin("12"), false);
        assert_eq!(validate_pin("123"), false);
        assert_eq!(validate_pin("12345"), false);
        assert_eq!(validate_pin("1234567"), false);
        assert_eq!(validate_pin("-1234"), false);
        assert_eq!(validate_pin("1.234"), false);
        assert_eq!(validate_pin("-1.234"), false);
        assert_eq!(validate_pin("00000000"), false);
    }

    #[test]
    fn non_digit_chars_tests() {
        assert_eq!(validate_pin("a234"), false);
        assert_eq!(validate_pin(".234"), false);
    }

    #[test]
    fn valid_pin_tests() {
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin("0000"), true);
        assert_eq!(validate_pin("1111"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("098765"), true);
        assert_eq!(validate_pin("000000"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("090909"), true);
    }

    fn find_short_dotest(s: &str, expected: u32) {
        let actual = find_short(s);
        assert!(actual == expected,
                "With s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn find_short_tests() {
        find_short_dotest("bitcoin take over the world maybe who knows perhaps", 3);
        find_short_dotest("turns out random test cases are easier than writing out basic ones", 3);
        find_short_dotest("lets talk about javascript the best language", 3);
        find_short_dotest("i want to travel the world writing code one day", 1);
        find_short_dotest("Lets all go on holiday somewhere very cold", 2);
        find_short_dotest("Let's travel abroad shall we", 2);
    }

    #[test]
    fn returns_expected() {
        assert_eq!(
            alphabet_position("The sunset sets at twelve o' clock."),
            "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
        );
        assert_eq!(
            alphabet_position("The narwhal bacons at midnight."),
            "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
        );
    }

    #[test]
    fn walk_tests() {
        assert!(is_valid_walk(&['n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's']));
        assert!(!is_valid_walk(&['w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e']));
        assert!(!is_valid_walk(&['w']));
        assert!(!is_valid_walk(&['n', 'n', 'n', 's', 'n', 's', 'n', 's', 'n', 's']));
        assert!(!is_valid_walk(&['e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's']))
    }

    fn readable_time_dotest(s: u32, expected: &str) {
        assert_eq!(make_readable(s), expected, "{ERR_MSG} with seconds = {s}")
    }

    #[test]
    fn readable_time_tests() {
        readable_time_dotest(0, "00:00:00");
        readable_time_dotest(59, "00:00:59");
        readable_time_dotest(60, "00:01:00");
        readable_time_dotest(3599, "00:59:59");
        readable_time_dotest(3600, "01:00:00");
        readable_time_dotest(86399, "23:59:59");
        readable_time_dotest(86400, "24:00:00");
        readable_time_dotest(359999, "99:59:59");
    }

    #[test]
    fn basic() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }


    fn to_camel_case_dotest(s: &str, expected: &str) {
        assert_eq!(to_camel_case(s), expected, "{ERR_MSG} with text = \"{s}\"")
    }

    #[test]
    fn to_camel_case_tests() {
        to_camel_case_dotest("", "");
        to_camel_case_dotest("the_stealth_warrior", "theStealthWarrior");
        to_camel_case_dotest("The-Stealth-Warrior", "TheStealthWarrior");
        to_camel_case_dotest("A-B-C", "ABC");
    }

    #[test]
    fn sample() {
        two_sum_do_test(&[1, 2, 3], 4);
        two_sum_do_test(&[1234, 5678, 9012], 14690);
        two_sum_do_test(&[2, 2, 3], 4);
    }

    fn two_sum_do_test(nums: &[i32], sum: i32) {
        let len = nums.len();
        let user_tuple = two_sum(nums, sum);
        assert!(
            user_tuple.0 < len && user_tuple.1 < len,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple has an index out of bounds",
            nums, sum, user_tuple
        );
        assert_ne!(user_tuple.0, user_tuple.1, "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple must have two different indices", nums, sum, user_tuple);
        let num1 = nums[user_tuple.0];
        let num2 = nums[user_tuple.1];
        let user_sum = num1 + num2;
        assert_eq!(user_sum, sum, "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nnumber as index {}: {}\nnumber as index {}: {}\nsum of the two numbers: {}\nsum of the two numbers did not equal target", nums, sum, user_tuple, user_tuple.0, num1, user_tuple.1, num2, user_sum)
    }

    fn rot13_dotest(s: &str, expected: &str) {
        assert_eq!(rot13(s), expected, "{ERR_MSG} with message = \"{s}\"")
    }

    #[test]
    fn rot13_sample_tests() {
        rot13_dotest("tesz", "grfm");
        rot13_dotest("Test", "Grfg");
    }

    #[test]
    fn tower_builder_tests() {
        assert_eq!(tower_builder(1), vec!["*"]);
        assert_eq!(tower_builder(2), vec![" * ", "***"]);
        assert_eq!(tower_builder(3), vec!["  *  ", " *** ", "*****"]);
    }

    fn parts_sums_dotest(ls: Vec<u64>, expect: Vec<u64>) {
        let actual = parts_sums(&ls);
        assert_eq!(actual, expect);
    }

    #[test]
    fn example() {
        parts_sums_dotest(vec![], vec![0]);
        parts_sums_dotest(vec![0, 1, 3, 6, 10], vec![20, 20, 19, 16, 10, 0]);
        parts_sums_dotest(vec![1, 2, 3, 4, 5, 6], vec![21, 20, 18, 15, 11, 6, 0]);
        parts_sums_dotest(vec![744125, 935, 407, 454, 430, 90, 144, 6710213, 889, 810, 2579358],
               vec![10037855, 9293730, 9292795, 9292388, 9291934, 9291504, 9291414, 9291270, 2581057, 2580168, 2579358, 0]);
    }

    fn valid_isbn10_dotest(isbn: &str, expected: bool) {
        let actual = valid_isbn10(isbn);
        assert!(actual == expected, "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn valid_isbn10_tests() {
        valid_isbn10_dotest("1112223339", true);
        valid_isbn10_dotest("048665088X", true);
        valid_isbn10_dotest("1293000000", true);
        valid_isbn10_dotest("1234554321", true);
        valid_isbn10_dotest("1234512345", false);
        valid_isbn10_dotest("1293", false);
        valid_isbn10_dotest("X123456788", false);
        valid_isbn10_dotest("ABCDEFGHIJ", false);
        valid_isbn10_dotest("XXXXXXXXXX", false);
        valid_isbn10_dotest("123456789T", false);
    }

    #[test]
    fn zeros_tests() {
        assert_eq!(zeros(0), 0);
        assert_eq!(zeros(5), 1);
        assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }

    fn decompose_testing(n: i64, exp: Option<Vec<i64>>) -> () {
        assert_eq!(decompose(n), exp)
    }

    #[test]
    fn tests_decompose() {

        decompose_testing(50, Some(vec![1,3,5,8,49]));
        decompose_testing(44, Some(vec![2,3,5,7,43]));
        decompose_testing(625, Some(vec![2,5,8,34,624]));
        decompose_testing(5, Some(vec![3,4]));

    }
}