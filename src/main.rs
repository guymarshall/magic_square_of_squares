use std::collections::HashMap;
use std::process::exit;
use std::time::Instant;

const LIMIT: usize = 1000;

// const fn generate_square_numbers<const LIMIT: usize>() -> [usize; LIMIT] {
//     let mut numbers: [usize; LIMIT] = [0usize; LIMIT];

//     let mut counter: usize = 0;
//     while counter < LIMIT {
//         numbers[counter] = (counter + 1) * (counter + 1);
//         counter += 1;
//     }

//     numbers
// }

pub(crate) fn get_most_frequent_total(square_numbers: &Vec<usize>) -> usize {
    const TOTAL_ITERATIONS: usize = LIMIT * LIMIT * LIMIT;
    let mut current_iteration: usize = 0;
    let mut totals_and_counts: HashMap<usize, usize> = HashMap::new();

    for first in square_numbers {
        for second in square_numbers {
            for third in square_numbers {
                let total: usize = first + second + third;
                let count: &mut usize = totals_and_counts.entry(total).or_insert(0);
                *count += 1;

                current_iteration += 1;
                let progress: f64 = (current_iteration as f64 / TOTAL_ITERATIONS as f64) * 100.0;

                if current_iteration % (TOTAL_ITERATIONS / 1000) == 0 {
                    println!("Getting most frequent total: {:.1}%", progress);
                }
            }
        }
    }

    let total_with_highest_count: usize = totals_and_counts
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(&total, _)| total)
        .unwrap();

    total_with_highest_count
}

#[inline(always)]
pub(crate) fn numbers_are_unique(numbers: &[usize; 9]) -> bool {
    numbers[0] != numbers[1]
        && numbers[0] != numbers[2]
        && numbers[0] != numbers[3]
        && numbers[0] != numbers[4]
        && numbers[0] != numbers[5]
        && numbers[0] != numbers[6]
        && numbers[0] != numbers[7]
        && numbers[0] != numbers[8]
        && numbers[1] != numbers[2]
        && numbers[1] != numbers[3]
        && numbers[1] != numbers[4]
        && numbers[1] != numbers[5]
        && numbers[1] != numbers[6]
        && numbers[1] != numbers[7]
        && numbers[1] != numbers[8]
        && numbers[2] != numbers[3]
        && numbers[2] != numbers[4]
        && numbers[2] != numbers[5]
        && numbers[2] != numbers[6]
        && numbers[2] != numbers[7]
        && numbers[2] != numbers[8]
        && numbers[3] != numbers[4]
        && numbers[3] != numbers[5]
        && numbers[3] != numbers[6]
        && numbers[3] != numbers[7]
        && numbers[3] != numbers[8]
        && numbers[4] != numbers[5]
        && numbers[4] != numbers[6]
        && numbers[4] != numbers[7]
        && numbers[4] != numbers[8]
        && numbers[5] != numbers[6]
        && numbers[5] != numbers[7]
        && numbers[5] != numbers[8]
        && numbers[6] != numbers[7]
        && numbers[6] != numbers[8]
        && numbers[7] != numbers[8]
}

fn main() {
    let start_time: Instant = Instant::now();

    // const SQUARE_NUMBERS: [usize; LIMIT] = generate_square_numbers();
    let square_numbers: Vec<usize> = (0..LIMIT)
        .map(|number: usize| (number + 1) * (number + 1))
        .collect();

    let most_frequent_total: usize = get_most_frequent_total(&square_numbers);

    println!("The most frequent total is {}", most_frequent_total);

    const TOTAL_ITERATIONS: usize = LIMIT * LIMIT * LIMIT;
    let mut current_iteration: usize = 0;
    let mut triplets_that_make_total: Vec<[usize; 3]> = vec![];

    for first in &square_numbers {
        for second in &square_numbers {
            for third in &square_numbers {
                let total: usize = first + second + third;

                if total == most_frequent_total {
                    triplets_that_make_total.push([*first, *second, *third]);
                }

                current_iteration += 1;
                let progress: f64 = (current_iteration as f64) / (TOTAL_ITERATIONS as f64) * 100.0;

                if current_iteration % (TOTAL_ITERATIONS / 1000) == 0 {
                    println!("Generating triplets: {:.1}%", progress)
                }
            }
        }
    }

    let count: usize = triplets_that_make_total.len();
    for (index, top_row) in triplets_that_make_total.clone().into_iter().enumerate() {
        for middle_row in &triplets_that_make_total {
            for bottom_row in &triplets_that_make_total {
                // don't need to check row sums as they are already correct

                // columns
                if top_row[0] + middle_row[0] + bottom_row[0] != most_frequent_total {
                    continue;
                }
                if top_row[1] + middle_row[1] + bottom_row[1] != most_frequent_total {
                    continue;
                }
                if top_row[2] + middle_row[2] + bottom_row[2] != most_frequent_total {
                    continue;
                }

                // diagonals
                if top_row[0] + middle_row[1] + bottom_row[2] != most_frequent_total {
                    continue;
                }
                if top_row[2] + middle_row[1] + bottom_row[0] != most_frequent_total {
                    continue;
                }

                let merged_rows: [usize; 9] = [
                    top_row[0],
                    top_row[1],
                    top_row[2],
                    middle_row[0],
                    middle_row[1],
                    middle_row[2],
                    bottom_row[0],
                    bottom_row[1],
                    bottom_row[2],
                ];

                if !numbers_are_unique(&merged_rows) {
                    continue;
                }

                println!("{:?}", merged_rows);
                exit(0);
            }
        }

        println!("Checking triples: {} / {}", index + 1, count);
    }

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
