use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

struct Graph {
    ch_node_map: HashMap<char, Rc<RefCell<Node>>>,
    ch_index_map: HashMap<char, usize>,
    index_ch_map: Vec<char>,
}


struct Node {
    c: char,
    edges: Vec<Rc<RefCell<Node>>>,
    in_edge: u32,
    visited: bool,
}

impl Node {
    fn new(c_ref: &char) -> Self {
        Self {
            c: c_ref.clone(),
            edges: vec![],
            in_edge: 0,
            visited: false,
        }
    }
}

fn floyd(graph: &mut Vec<Vec<i32>>, size: usize) -> Vec<Vec<usize>> {
    //must be matrix
    assert_eq!(graph.len(), size);
    for v in graph.iter() {
        assert_eq!(v.len(), size);
    }
    let mut matrix_s = vec![vec![0; size]; size];

    //init s
    for i in 0..size {
        for j in 0..size {
            if i != j {
                matrix_s[i][j] = j;
            }
        }
    }

    for i in 0..size {
        for j in 0..size {
            if i == j {
                continue;
            }
            for k  in 0..size {
                if i == k || j == k {
                    continue;
                }
                let ik_d = graph[i][k];
                let kj_d = graph[k][j];
                if  graph[i][k] != i32::MAX && graph[k][j] != i32::MAX {
                    let new_distance = graph[i][k] + graph[k][j];
                    if new_distance < graph[i][j] || graph[i][j] == i32::MAX {
                        graph[i][j] = graph[i][k] + graph[k][j];
                        matrix_s[i][j] = k;
                    }
                }
            }
        }
    }

    matrix_s
}

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut g = Graph {
        ch_node_map: HashMap::new(),
        ch_index_map: HashMap::new(),
        index_ch_map: vec![]
    };
    for triple in triplets.iter() {
        let [first, second, third] = triple;
        let mut tr;
        match g.ch_node_map.get(third) {
            None => {
                tr = Rc::new(RefCell::new(Node::new(third)));
                g.ch_node_map.insert(*third, tr.clone());
                g.ch_index_map.insert(*third, g.index_ch_map.len());
                g.index_ch_map.push(*third);
            }
            Some(v) => { tr = v.clone() }
        }
        tr.borrow_mut().in_edge += 1;
        let mut sr;
        match g.ch_node_map.get_mut(second) {
            None => {
                sr = Rc::new(RefCell::new(Node::new(second)));
                sr.borrow_mut().edges.push(tr);
                g.ch_node_map.insert(*second, sr.clone());
                g.ch_index_map.insert(*second, g.index_ch_map.len());
                g.index_ch_map.push(*second);
            }
            Some(v) => {
                v.borrow_mut().edges.push(tr);
                sr = v.clone();
            }
        }
        sr.borrow_mut().in_edge += 1;
        match g.ch_node_map.get_mut(first) {
            None => {
                let mut fr = Rc::new(RefCell::new(Node::new(first)));
                fr.borrow_mut().edges.push(sr);
                g.ch_node_map.insert(*first, fr.clone());
                g.ch_index_map.insert(*first, g.index_ch_map.len());
                g.index_ch_map.push(*first);
            }
            Some(v) => {
                v.borrow_mut().edges.push(sr);
            }
        }
    }

    let (_, start_node) = g.ch_node_map.iter()
        .filter(|(_, node)| node.borrow().in_edge == 0)
        .last().unwrap();

    let start = start_node.clone();
    let start_char = start.borrow().c;
    let start_num = *g.ch_index_map.get(&start_char).unwrap();
    let node_num = g.index_ch_map.len();

    let mut matrix_d = vec![vec![i32::MAX; node_num]; node_num];
    //init graph
    for (c, r) in &g.ch_node_map {
        let from_node_index = *g.ch_index_map.get(c).unwrap();
        for to_node in &r.borrow().edges {
            let to_node_char = &to_node.borrow().c;
            let to_node_index = *g.ch_index_map.get(to_node_char).unwrap();
            matrix_d[from_node_index][to_node_index] = -1;
        }
    }

    let matrix_s = floyd(&mut matrix_d, node_num);

    //[    p   u   t   i   h   w   s   a
    // p [ -,  -,  -,  -,  -,  -,  -,  -],
    // u [-1,  -,  -,  -,  -,  -,  -,  -],
    // t [-2, -2,  -, -1,  -,  -, -2,  -],
    // i [ -, -2,  -,  -,  -,  -, -1,  -],
    // h [-2, -3, -2, -3,  -,  -, -4, -1],
    // w [-3, -4, -3, -4, -1,  -, -5, -2],
    // s [-2, -1,  -,  -,  -,  -,  -,  -],
    // a [-3, -3, -1, -2,  -,  -, -3,  -]
    // ];
    // [
    // [0, 1, 2, 3, 4, 5, 6, 7],
    // [0, 0, 2, 3, 4, 5, 6, 7],
    // [1, 6, 0, 3, 4, 5, 3, 7],
    // [0, 6, 2, 0, 4, 5, 6, 7],
    // [7, 3, 7, 2, 0, 5, 2, 7],
    // [4, 4, 4, 2, 4, 0, 2, 4],
    // [1, 1, 2, 3, 4, 5, 0, 7],
    // [2, 2, 2, 2, 4, 5, 2, 0]
    // ]
    return format!("{:?};{:?}", matrix_d, matrix_s);
    let (end_num, value) = matrix_d[start_num].iter().enumerate().min_by(|(_, &a), (_, &b)| {
        if a > b {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }).unwrap();
    let mut p_num = start_num;
    let mut q_num = end_num;
    let mut tail_index_array = find_path(p_num, q_num, &matrix_s);
    let mut index_array = vec![start_num];
    index_array.append(&mut tail_index_array);
    index_array.into_iter().map(|x| g.index_ch_map[x]).collect()
}

fn find_path(p: usize, q: usize, s: &Vec<Vec<usize>>) -> Vec<usize> {
    let m = s[p][q];
    return if m == q {
        vec![m]
    } else {
        let mut left = find_path(p, m, s);
        let mut right = find_path(m, q, s);
        left.append(&mut right);
        left
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn node_test() {
        let n = Node::new(&'a');
    }

    #[test]
    fn floyd_test() {
        let m = i32::MAX;
        let mut graph = vec![
           vec![0, 2, m, 6],
           vec![2, 0, 3, 2],
           vec![m, 3, 0, 2],
           vec![6, 2, 2, 0]
        ];
        let path = floyd(&mut graph, 4);

        assert_eq!(graph, vec![
            vec![0, 2, 5, 4],
            vec![2, 0, 3, 2],
            vec![5, 3, 0, 2],
            vec![4, 2, 2, 0]
        ], "graph failed");

        assert_eq!(path, vec![
            vec![0, 1, 1, 1],
            vec![0, 0, 2, 3],
            vec![1, 1, 0, 3],
            vec![1, 1, 2, 0]
        ], "path failed");

    }

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