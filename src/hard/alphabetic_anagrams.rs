use std::collections::HashMap;

//TODO can not understand
fn list_position(word: &str) -> u128 {
    let (mut r, mut s, l, mut c) = (1u128, 1u128, word.len(), HashMap::new());
    for i in 0..l {
        let x = word.chars().nth(l-1-i).unwrap();
        c.entry(x.to_owned()).and_modify(|m|{*m += 1}).or_insert(1);
        let d = c.get(&x).unwrap();
        for (y, e) in &c {
            if *y < x {
                r += s * e / d;
            }
        }
        s = s * (i+1) as u128 / d;
    }
    r
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::list_position;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    #[test]
    fn sample_tests() {
        let test_data = [
            (                  "A", 1),
            (               "ABAB", 2),
            (               "AAAB", 1),
            //AB
            //A
            (               "BAAA", 4),
            //MMYY
            //MYMY
            //MYYM
            //YMMY
            //YMYM 5
            //YYMM


            //MM 1
            //MY 2
            //YM
            //YMM 1
            //YMY 1
            //YMYM
            (               "YMYM", 5),
            //QUESTION == 81, 85, 69, 83, 84, 73, 79, 78
            //69, 73, 78, 79, /81, 83, 84, 85
            //73, ...
            //78, ...
            //79, ...
            //A(7,7) * 4
            //69, 73, 78, 79, 83, 84, |85
            //81, 69, ...
            //81, 84, ...
            //A(6, 6) * 6
            //|69, 73, 78, 79, 83, 84
            //0
            //73, 78, 79, |83, 84
            //A(4, 4) * 3
            //73, 78, 79, |84
            //A(3, 3) * 3
            //|73, 78, 79
            //0
            //78, |79
            //1
            //79
            //1
            //A(7,7) * 4 + A(6, 6) * 6 + A(4, 4) * 3 + A(3, 3) * 3 + 1 + 1
            //
            (           "QUESTION", 24572),
            // BOOKKEEPER = 66, 79, 79, 75, 75, 69, 69, 80, 69, 82
            // sorted 66,  69,  69,  69,  75,  75,  |79, 79,  80,  82
            //        'B', 'E', 'E', 'E', 'K', 'K', 'O', 'O', 'P', 'R'
            // deduped 66, 69, 75, |79, 80, 82

            //BE 8! / (2 * 2 * 2) 5040
            //BK 8! / (3! * 2) 3360
            //BO \

            //BOE 7! / (2 * 2) 1260
            //BOK 7! / (3!) 840
            //BOO

            //BOOE 6! / (2 * 2) 180
            //BOOK

            //BOOKE 5! / (2) 60
            //BOOKK

            //BOOKKE 4!

            //BOOKKEE 3!

            //BOOKKEEE 2
            //BOOKKEEP

            //BOOKKEEPE
            //DONE
            (         "BOOKKEEPER", 10743),
            ("IMMUNOELECTROPHORETICALLY", 718393983731145698173),
        ];
        for (word, expected) in test_data {
            assert_eq!(list_position(word),
                       expected,
                       "\nYour result (left) did not match the expected output (right) for the input: \"{word}\"");
        }

    }
}