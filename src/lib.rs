use std::cmp::min;

pub fn valid_tours(b: &[u32]) -> u32 {
    if b.is_empty() {
        return 0;
    }

    let mut t = Vec::new();

    for (day, _rest_days_needed) in b.iter().enumerate() {
        t.push(1);

        if day == 0 {
            // Base case
            continue;
        }

        for prev_days in 0..day {
            if b[prev_days] + (prev_days as u32) < day as u32 {
                t[day] += t[prev_days];
            }
        }
    }

    t.iter().sum()
}

#[derive(Clone)]
struct DuneMerge {
    cost: u32,
    new_dune_size: u32,
}

pub fn sand_dunes_merging(cost: &[u32]) -> u32 {
    if cost.is_empty() {
        return 0;
    }

    let mut t = vec![
        vec![
            DuneMerge {
                cost: u32::MAX,
                new_dune_size: 0
            };
            cost.len()
        ];
        cost.len()
    ];

    // merging `merge_count` other elements to current dune
    for merge_count in 0..cost.len() {
        for (index, dune_size) in cost.iter().enumerate() {
            if index + merge_count >= cost.len() {
                break;
            }

            // Base case - dune alone
            if merge_count == 0 {
                t[index][index].cost = *dune_size;
                t[index][index].new_dune_size = *dune_size;
                continue;
            }

            for sub_merge in 0..merge_count {
                let mut cost: u32 = 0;
                let left_side_start_index = index;
                let left_side_end_index = index + sub_merge;
                let left_side_cost = t[left_side_start_index][left_side_end_index].cost;
                let left_side_new_dune_size =
                    t[left_side_start_index][left_side_end_index].new_dune_size;
                if left_side_start_index != left_side_end_index {
                    cost = cost.saturating_add(left_side_new_dune_size);
                }

                let right_side_start_index = index + sub_merge + 1;
                let right_side_end_index = index + merge_count;
                let right_side_cost = t[right_side_start_index][right_side_end_index].cost;
                let right_side_new_dune_size =
                    t[right_side_start_index][right_side_end_index].new_dune_size;
                if right_side_start_index != right_side_end_index {
                    cost = cost.saturating_add(right_side_new_dune_size);
                }

                t[left_side_start_index][right_side_end_index].cost = min(
                    t[index][index + merge_count].cost,
                    cost.saturating_add(left_side_cost)
                        .saturating_add(right_side_cost),
                );
                t[left_side_start_index][right_side_end_index].new_dune_size =
                    left_side_new_dune_size.saturating_add(right_side_new_dune_size);
            }
        }
    }

    t[0][cost.len() - 1].cost
}

pub fn sand_dunes_arbitrary_cost_merging(sand_dunes: &[u32], cost: &[&[&[u32]]]) -> u32 {
    if sand_dunes.is_empty() {
        return 0;
    }

    let mut t = vec![vec![u32::MAX; sand_dunes.len()]; sand_dunes.len()];

    // merging `merge_count` other elements to current dune
    for merge_count in 0..sand_dunes.len() {
        for (index, _) in sand_dunes.iter().enumerate() {
            if index + merge_count >= sand_dunes.len() {
                break;
            }

            // Base case - dune alone
            if merge_count == 0 {
                t[index][index] = cost[index][index][index];
                continue;
            }

            for sub_merge in 0..merge_count {
                let left_side_start_index = index;
                let left_side_end_index = left_side_start_index + sub_merge;
                let right_side_start_index = left_side_end_index + 1;
                let right_side_end_index = index + merge_count;

                t[left_side_start_index][right_side_end_index] = min(
                    t[index][index + merge_count],
                    t[left_side_start_index][left_side_end_index]
                        + t[right_side_start_index][right_side_end_index]
                        + cost[left_side_start_index][left_side_end_index][right_side_end_index],
                );
            }
        }
    }

    t[0][sand_dunes.len() - 1]
}

pub fn greedy_sand_dune_merging(cost: &[u32]) -> u32 {
    let mut merged = cost.to_vec();
    let mut cost = 0;

    while merged.len() > 1 {
        let mut consecutive_sum = u32::MAX;
        let mut min_index = 0;

        // Find smallest two items
        for (index, dune) in merged.iter().enumerate() {
            if index + 1 < merged.len() && dune + merged[index + 1] < consecutive_sum {
                min_index = index;
                consecutive_sum = dune + merged[index + 1];
            }
        }

        // Merge
        merged[min_index] = consecutive_sum;
        merged.remove(min_index + 1);
        cost += consecutive_sum;
    }

    cost
}

fn get_line<T: AsRef<str>>(word_list: &[T], start_index: usize, end_index: usize) -> String {
    word_list[start_index..=end_index]
        .iter()
        .map(|item| item.as_ref())
        .collect::<Vec<&str>>()
        .join(" ")
}

fn calculate_penalty<T: AsRef<str>>(
    word_list: &[T],
    start_index: usize,
    end_index: usize,
    limit: u32,
) -> u32 {
    let line = get_line(word_list, start_index, end_index);
    let diff_count = (line.len() as u32).abs_diff(limit);

    diff_count.pow(2)
}

pub fn word_wrapper<T: AsRef<str>>(a: &[T], m: u32) -> u32 {
    let mut t: Vec<u32> = Vec::new();

    for index in (0..a.len()).rev() {
        // Base case - last line
        if get_line(a, index, a.len() - 1).len() <= m as usize {
            t.insert(0, 0);
            continue;
        }

        // Start with the penalty of having just this word on a single line,
        // with the best case penalty for the following lines
        let mut penalty = calculate_penalty(a, index, index, m) + t[0];

        for next_line_word_index in index + 1..a.len() - 1 {
            penalty = min(
                penalty,
                calculate_penalty(a, index, next_line_word_index, m)
                    + t[next_line_word_index - index],
            );
        }
        t.insert(0, penalty);
    }

    t[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tours_handout() {
        let result = valid_tours(&[1, 1, 5]);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_valid_tours_extra_cases() {
        assert_eq!(valid_tours(&[0, 0]), 3);
        assert_eq!(valid_tours(&[0, 0, 0, 0]), 15);
        assert_eq!(valid_tours(&[1, 1, 1, 1]), 7);
        assert_eq!(valid_tours(&[1, 1, 1, 1, 1]), 12);
    }

    #[test]
    fn test_sand_dunes_merging() {
        assert_eq!(sand_dunes_merging(&[1, 1]), 2);
        assert_eq!(sand_dunes_merging(&[1, 1, 1]), 5);
        assert_eq!(sand_dunes_merging(&[3, 5, 1]), 15);
        assert_eq!(sand_dunes_merging(&[1, 1, 1, 1]), 8);
        assert_eq!(sand_dunes_merging(&[10, 1, 1, 10]), 36);
    }

    #[test]
    fn test_sand_dunes_merging_meet_patel() {
        assert_eq!(sand_dunes_merging(&[4, 8, 6, 3, 1, 9]), 76);
    }

    #[test]
    fn test_greedy_sand_dunes_merging() {
        assert_eq!(greedy_sand_dune_merging(&[1, 1]), 2);
        assert_eq!(greedy_sand_dune_merging(&[1, 1, 1]), 5);
        assert_eq!(greedy_sand_dune_merging(&[3, 5, 1]), 15);
        assert_eq!(greedy_sand_dune_merging(&[1, 1, 1, 1]), 8);
        assert_eq!(greedy_sand_dune_merging(&[10, 1, 1, 10]), 36);
    }

    #[test]
    fn test_different_merge_strategy() {
        assert_eq!(sand_dunes_merging(&[4, 8, 2, 8]), 44);
        assert_eq!(greedy_sand_dune_merging(&[4, 8, 2, 8]), 46);
    }

    #[test]
    fn test_word_wrapper_handout() {
        assert_eq!(
            word_wrapper(
                &[
                    "Check",
                    "out",
                    "these",
                    "9",
                    "hilarious",
                    "things",
                    "Purdue",
                    "Pete",
                    "was",
                    "caught",
                    "doing.",
                    "I",
                    "couldn't",
                    "believe",
                    "number",
                    "5!"
                ],
                16
            ),
            3
        )
    }

    #[test]
    fn test_word_wrapper_extra_cases() {
        assert_eq!(word_wrapper(&["number", "5!"], 16), 0);
    }

    #[test]
    fn test_word_wrapper_ed_post() {
        assert_eq!(word_wrapper(&["dddd", "eeee"], 4), 0);
        assert_eq!(word_wrapper(&["cc", "dddd", "eeee"], 4), 4);
        assert_eq!(word_wrapper(&["bbbb", "cc", "dddd", "eeee"], 4), 4);
        assert_eq!(word_wrapper(&["aa", "bbbb", "cc", "dddd", "eeee"], 4), 8);
    }
}
