enum Direction { Up, Down, Left, Right }

fn test(r: &Vec<Vec<i8>>, d: &Direction, i: usize, j: usize) -> Option<Direction> {
    let fuck = r.len() - 3;
    let mut just_continue = false;
    let temp_d = match d {
        Direction::Up => {
            if  ((i < 2 && r[0][j] == 1) || (i >= 2 && r[i - 2][j] == 1)) && r[i + 2][j] != 1 {
                None
            } else if i < 2 && r[0][j] == 1 {
                Some(Direction::Right)
            } else if i >= 2 && r[i - 2][j] == 1 {
                Some(Direction::Right)
            } else {
                just_continue = true;
                Some(Direction::Up)
            }
        }
        Direction::Down => {
            if  ((i > fuck && r[r.len() - 1][j] == 1) || (i <= fuck && r[i + 2][j] == 1)) && r[i - 2][j] != 1 {
                None
            } else if i > fuck && r[r.len() - 1][j] == 1 {
                Some(Direction::Left)
            } else if i <= fuck && r[i + 2][j] == 1 {
                Some(Direction::Left)
            } else {
                just_continue = true;
                Some(Direction::Down)
            }
        }
        Direction::Left => {
            if  ((j < 2 && r[i][0] == 1) || (j >= 2 && r[i][j - 2] == 1)) && r[i][j + 2] != 1 {
                None
            } else if j < 2 && r[i][0] == 1 {
                Some(Direction::Up)
            } else if j >= 2 && r[i][j - 2] == 1 {
                Some(Direction::Up)
            } else {
                just_continue = true;
                Some(Direction::Left)
            }
        }
        Direction::Right => {
            if  ((j > fuck && r[i][r.len() - 1] == 1) || (j <= fuck && r[i][j + 2] == 1)) && r[i][j - 2] != 1 {
                None
            } else if j > fuck && r[i][r.len() - 1] == 1 {
                Some(Direction::Down)
            } else if j <= fuck && r[i][j + 2] == 1 {
                Some(Direction::Down)
            } else {
                just_continue = true;
                Some(Direction::Right)
            }
        }
    };

    if temp_d.is_none() {
        return None;
    } else if just_continue {
        return temp_d
    } else if !just_continue {
        return match temp_d.unwrap() {
            Direction::Up => {
                if i < 2 && r[0][j] == 1 {
                    None
                } else if i >= 2 && r[i - 2][j] == 1 {
                    None
                } else {
                    just_continue = true;
                    Some(Direction::Up)
                }
            }
            Direction::Down => {
                if i > fuck && r[r.len() - 1][j] == 1 {
                    None
                } else if i <= fuck && r[i + 2][j] == 1 {
                    None
                } else {
                    just_continue = true;
                    Some(Direction::Down)
                }
            }
            Direction::Left => {
                if j < 2 && r[i][0] == 1 {
                    None
                } else if j >= 2 && r[i][j - 2] == 1 {
                    None
                } else {
                    just_continue = true;
                    Some(Direction::Left)
                }
            }
            Direction::Right => {
                if j > fuck && r[i][r.len() - 1] == 1 {
                    None
                } else if j <= fuck && r[i][j + 2] == 1 {
                    None
                } else {
                    just_continue = true;
                    Some(Direction::Right)
                }
            }
        };
    }


    None
}

fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut result = vec![vec![0; size]; size];
    result[0][0] = 1;

    let mut d = Direction::Right;
    let mut i = 0;
    let mut j = 0;

    loop {
        match d {
            Direction::Up => {
                i -= 1;
            }
            Direction::Down => {
                i += 1;
            }
            Direction::Left => {
                j -= 1;
            }
            Direction::Right => {
                j += 1;
            }
        }
        result[i][j] = 1;
        match test(&result, &d, i, j) {
            None => { break; }
            Some(nd) => { d = nd; }
        }
    }
    result
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
