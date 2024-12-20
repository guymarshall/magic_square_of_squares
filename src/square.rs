#[inline(always)]
pub fn sums_are_valid(
    top_row: &[usize; 3],
    middle_row: &[usize; 3],
    bottom_row: &[usize; 3],
    most_frequent_total: usize,
) -> bool {
    // don't need to check row sums as they are already correct

    // columns
    if top_row[0] + middle_row[0] + bottom_row[0] != most_frequent_total {
        return false;
    }
    if top_row[1] + middle_row[1] + bottom_row[1] != most_frequent_total {
        return false;
    }
    if top_row[2] + middle_row[2] + bottom_row[2] != most_frequent_total {
        return false;
    }

    // diagonals
    if top_row[0] + middle_row[1] + bottom_row[2] != most_frequent_total {
        return false;
    }
    if top_row[2] + middle_row[1] + bottom_row[0] != most_frequent_total {
        return false;
    }

    true
}
