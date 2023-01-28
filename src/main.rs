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
    let mut pair;
    let mut marks = vec![true; arr.len()];
    let mut marks_changed = false;
    if arr.len() < 2 { return arr.iter().map(|d| *d).collect() }
    loop {
        marks_changed = false;
        let mut i = 0;
        let mut j = 1;
        pair = (arr[i], arr[j]);
        match pair {
            (Direction::North, Direction::South)
            | (Direction::South, Direction::North)
            | (Direction::West, Direction::East)
            | (Direction::East, Direction::West) => {
                marks[i] = false; marks[j] = false;
            }
            _ => {}
        }
        if j + 1 > arr.len() - 1 {
            if marks_changed {
                i = 0; j = 1;
            } else {
                break;
            }
        }
        i+= 1; j+= 1;
        while !marks[i] && i + 1 < arr.len() - 1 {
            i += 1;
        }

        while j < i && j < arr.len() - 1 {
            j += 1;
        }

        if i == j {
            break;
        }
    }

    arr.iter()
        .enumerate()
        .filter(|(i, _)| marks[*i])
        .map(|(_, d)| *d)
        .collect()
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
        assert!(  is_valid_walk(&['n','s','n','s','n','s','n','s','n','s']));
        assert!(! is_valid_walk(&['w','e','w','e','w','e','w','e','w','e','w','e']));
        assert!(! is_valid_walk(&['w']));
        assert!(! is_valid_walk(&['n','n','n','s','n','s','n','s','n','s']));
        assert!(! is_valid_walk(&['e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's']))
    }

    const RT_ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn readable_time_dotest(s: u32, expected: &str) {
        assert_eq!(make_readable(s), expected, "{RT_ERR_MSG} with seconds = {s}")
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
}