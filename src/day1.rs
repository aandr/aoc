mod utils;

pub fn main() {
    let input = utils::get_puzzle_input(1);

    let mut sums = vec![0];

    for row in input.split('\n') {
        if row.is_empty() {
            sums.push(0)
        } else {
            let Ok(row_value) = row.parse::<i32>() else {
                panic!("Failed to parse row: '{}'", row);
            };
            *sums.last_mut().unwrap() += row_value;
        }
    }

    let mut top3 = [0, 0, 0];

    for sum in sums {
        if sum > top3[0] {
            if sum > top3[1] {
                if sum > top3[2] {
                    top3[0] = top3[1];
                    top3[1] = top3[2];
                    top3[2] = sum;
                } else {
                    top3[0] = top3[1];
                    top3[1] = sum;
                }
            } else {
                top3[0] = sum;
            }
        }
    }

    println!("Max sum: {}", top3[2]);
    println!("Max 3 sum: {}", top3.iter().sum::<i32>());
}
