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

        let mut current  = vec![0; width];
        let mut previous = vec![0; width];

        for i in 0..width {
            previous[i] = i;
        }

        current[0] = previous[0] + 1;

        for row in 1..depth {
            for column in 1..width {
                if left[column - 1] == right[row - 1] {
                    current[column] = previous[column - 1];
                } else {
                    let above = previous[column]     + 1;
                    let diag  = previous[column - 1] + 1;
                    let aside = current [column - 1] + 1;

                    current[column] = *[above, diag, aside]
                        .iter()
                        .min()
                        .unwrap();
                }
            }

            previous = current.clone();
            current[0] = row+1;
        }

        previous[width-1]
    }
