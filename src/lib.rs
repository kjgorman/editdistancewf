#![crate_name = "editdistancewf"]
#![crate_type = "lib"]

fn materialise<T, I: Iterator<Item=T>>(iter: I) -> Vec<T> {
    let mut dest = Vec::new();
    dest.extend(iter);

    dest
}

pub fn distance<T, I: Iterator<Item=T>>(left_stream: I, right_stream: I) -> usize
    where T: Eq {
        let left  = materialise(left_stream);
        let right = materialise(right_stream);

        let width = left.len() + 1;
        let depth = right.len() + 1;

        let mut matrix = vec![vec![0; depth]; width];

        for i in 0..width {
            matrix[i][0] = i;
        }

        for j in 0..depth {
            matrix[0][j] = j;
        }

        for j in 1..depth {
            for i in 1..width {
                // these elements match, propagate current distances
                if left[i-1] == right[j-1] {
                    matrix[i][j] = matrix[i-1][j-1];
                } else {
                    matrix[i][j] = *[
                        matrix[i-1][j] + 1,    // deletion
                        matrix[i][j-1] + 1,    // insertion
                        matrix[i-1][j-1] + 1]  // substitution
                        .iter()
                        .min()
                        .unwrap();
                }
            }
        }

        matrix[width-1][depth-1]
    }
