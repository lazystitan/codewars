fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    use std::collections::HashMap;
    let mut index_char_map = vec![];
    let mut char_index_map = HashMap::new();
    for triple in &triplets {
        for c in triple {
            if !char_index_map.contains_key(c) {
                index_char_map.push(*c);
                char_index_map.insert(*c, index_char_map.len() - 1);
            }
        }
    }

    let size = index_char_map.len();
    let mut graph = vec![vec![0; size]; size];
    for tri in &triplets {
        let indexs: Vec<usize> = tri.iter().map(|x| *(char_index_map.get(x).unwrap())).collect();
        graph[indexs[0]][indexs[1]] = 1;
        graph[indexs[1]][indexs[2]] = 1;
        graph[indexs[0]][indexs[2]] = 1;
    }

    let mut index_collector = vec![];
    let mut n = size;
    while n > 0 {
        for i in 0..size  {
            if index_collector.contains(&i) {
                continue;
            }
            let mut no_in = true;
            for o in 0..size {
                if graph[o][i] != 0 {
                    no_in = false;
                    break;
                }
            }
            if no_in {
                index_collector.push(i);
                for j in 0..size {
                    graph[i][j] = 0;
                }
                n -= 1;
                break;
            }
        }
    }

    index_collector.into_iter().map(|x| index_char_map[x]).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn recover_secret_test() {
        assert_eq!(recover_secret(vec![
            ['t', 'u', 'p'],
            ['w', 'h', 'i'],
            ['t', 's', 'u'],
            ['a', 't', 's'],
            ['h', 'a', 'p'],
            ['t', 'i', 's'],
            ['w', 'h', 's']])
                   , "whatisup");
    }

    #[test]
    fn format_test() {
        assert_eq!(format!("{:?}", vec![vec![1; 10]; 10]), "")
    }
}