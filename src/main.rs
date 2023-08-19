use itertools::Itertools;

fn generate_square_numbers(count: i32) -> Vec<i32> {
    (0..=count).map(|number| number * number).collect()
}

fn rotate_vector(layout: &mut Vec<i32>) {
    let temp: i32 = layout[0];
    layout[0] = layout[2];
    layout[2] = layout[8];
    layout[8] = layout[6];
    layout[6] = temp;

    let temp: i32 = layout[1];
    layout[1] = layout[5];
    layout[5] = layout[7];
    layout[7] = layout[3];
    layout[3] = temp;
}

fn sums_are_equal(numbers: &Vec<i32>) -> bool {
    let top_row_sum: i32 = numbers[0] + numbers[1] + numbers[2];
    let middle_row_sum: i32 = numbers[3] + numbers[4] + numbers[5];
    let bottom_row_sum: i32 = numbers[6] + numbers[7] + numbers[8];

    if top_row_sum != middle_row_sum || middle_row_sum != bottom_row_sum {
        return false;
    }

    let left_column_sum: i32 = numbers[0] + numbers[3] + numbers[6];
    let middle_column_sum: i32 = numbers[1] + numbers[4] + numbers[7];
    let right_column_sum: i32 = numbers[2] + numbers[5] + numbers[8];

    if bottom_row_sum != left_column_sum || left_column_sum != middle_column_sum || middle_column_sum != right_column_sum {
        return false;
    }

    let nw_se_sum: i32 = numbers[0] + numbers[4] + numbers[8];
    let sw_ne_sum: i32 = numbers[6] + numbers[4] + numbers[2];

    right_column_sum == nw_se_sum && nw_se_sum == sw_ne_sum
}

fn main() {
    const LIMIT: i32 = 9;

    let square_numbers: Vec<i32> = generate_square_numbers(LIMIT);

    // generate 9! permutations
    // then for every iteration, plug in a different combination of indices
    // for the squares vector

    let indices: itertools::Permutations<std::ops::Range<usize>> = (0..9).permutations(9);

    println!("Generating combinations...");
    let combinations: itertools::Combinations<std::slice::Iter<'_, i32>> = square_numbers.iter().combinations(9);

    println!("Processing...");
    combinations.for_each(|combination: Vec<&i32>| {
        indices.clone().for_each(|index: Vec<usize>| {
            let data: Vec<i32> = index.iter().map(|&i| combination[i]).cloned().collect();

            if sums_are_equal(&data) {
                println!("{:?}", data);
            }
        });
    });
}
