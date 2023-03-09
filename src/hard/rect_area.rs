use std::collections::{BTreeSet, HashMap};

fn calculate(rectangles: &[[i32; 4]]) -> i64 {
    if rectangles.is_empty() {
        return 0;
    } else if rectangles.len() == 1 {
        let [x1, y1, x2, y2] =  rectangles[0];
        return ((x2 - x1) * (y2 - y1)) as i64
    }

    let mut x_axis_values = vec![];
    let mut y_axis_values = vec![];

    for &rt in rectangles {
        let [x0, y0, x1, y1] = rt;
        x_axis_values.push(x0);
        x_axis_values.push(x1);
        y_axis_values.push(y0);
        y_axis_values.push(y1);
    }
    x_axis_values.sort();
    x_axis_values.dedup();
    let x_axis_values_map = x_axis_values
        .iter()
        .enumerate()
        .map(|(index, &value)| (value, index))
        .collect::<HashMap<_, _>>();
    y_axis_values.sort();
    y_axis_values.dedup();
    let y_axis_values_map = y_axis_values
        .iter()
        .enumerate()
        .map(|(index, &value)| (value, index))
        .collect::<HashMap<_, _>>();
    let mut m = vec![vec![0; y_axis_values.len()]; x_axis_values.len()];

    for &rt in rectangles {
        let [x0, y0, x1, y1] = rt;
        let x0_index = *x_axis_values_map.get(&x0).unwrap();
        let x1_index = *x_axis_values_map.get(&x1).unwrap();
        let y0_index = *y_axis_values_map.get(&y0).unwrap();
        let y1_index = *y_axis_values_map.get(&y1).unwrap();
        for x in x0_index..x1_index {
            for y in y0_index..y1_index {
                let x_len = x_axis_values[x + 1] - x_axis_values[x];
                let y_len = y_axis_values[y + 1] - y_axis_values[y];
                let grid_size = x_len * y_len;
                m[x][y] = grid_size;
            }
        }
    }

    m.into_iter().fold(0, |s, line| {
        s + line.iter().map(|&x| x as i64).sum::<i64>()
    })
}
#[cfg(test)]
mod sample_tests {
    use super::calculate;

    const ERR_MSG : &str = "\nYour result (left) did not equal expected result (right)";

    #[test]
    fn zero_rectangles() {
        assert_eq!(calculate(&[]), 0, "{}", ERR_MSG);
    }

    #[test]
    fn one_rectangle() {
        assert_eq!(calculate(&[[0,0,1,1]]), 1, "{}", ERR_MSG);
        assert_eq!(calculate(&[[0,4,11,6]]), 22, "{}", ERR_MSG);
    }

    #[test]
    fn two_rectangles() {
        assert_eq!(calculate(&[[0,0,1,1], [1,1,2,2]]), 2, "{}", ERR_MSG);
        assert_eq!(calculate(&[[0,0,1,1], [0,0,2,2]]), 4, "{}", ERR_MSG);
    }

    #[test]
    fn three_rectangles() {
        assert_eq!(calculate(&[[3,3,8,5], [6,3,8,9], [11,6,14,12]]), 36, "{}", ERR_MSG);
    }
}
