use anyhow::{anyhow, Result};

/// Finds the maximum possible homework points given the homework details and end-of-term date
///
/// Note: the last day isn't counted in this implementation, as we assume each homework takes 1 unit
///       of time to complete!
pub fn homework_points(p: &[i32], t: &[u32], e: u16) -> Result<u32> {
    if p.len() != t.len() {
        return Err(anyhow!("Points and time arrays do not match in length!"));
    }

    if e == 0 {
        return Ok(0);
    }

    // (point, assigned_time)
    let mut homeworks: Vec<(i32, u32)> = Vec::new();

    for (point, time) in p.iter().zip(t.iter()) {
        homeworks.push((*point, *time));
    }

    let mut completed_homeworks = Vec::new();

    for day in 0..(e - 1) {
        // From the pool of possible homeworks (assigned on or before current `day` value), find max
        let possible_homeworks: Vec<(i32, u32)> = homeworks
            .iter()
            .cloned()
            .filter(|(_, d)| *d <= day as u32)
            .collect();
        let max_points_homework = possible_homeworks.iter().cloned().max_by_key(|h| h.0);

        // If such a homework satisfying the criteria above exists...
        if let Some(max_points_homework) = max_points_homework {
            if max_points_homework.0 < 0 {
                // Negative points. Skip doing this homework
                continue;
            }
            
            completed_homeworks.push(max_points_homework);
            // Remove completed homework from pool
            if let Some(index) = homeworks.iter().position(|&h| h == max_points_homework) {
                // `swap_remove` works because we don't care about the ordering; minor perf boost
                homeworks.swap_remove(index);
            }
        }
    }

    // Find sum of all points in completed homeworks
    Ok(completed_homeworks.iter().map(|(point, _)| point).sum::<i32>() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homework_points() {
        let p = vec![8, 2, 4, 5, 9, 6, 3];
        let t = vec![0, 4, 1, 6, 2, 0, 0];

        assert_eq!(homework_points(&p, &t, 0).unwrap(), 0);
        assert_eq!(homework_points(&p, &t, 1).unwrap(), 0);
        assert_eq!(homework_points(&p, &t, 2).unwrap(), 8);
        assert_eq!(homework_points(&p, &t, 3).unwrap(), 14);
        assert_eq!(homework_points(&p, &t, 4).unwrap(), 23);
        assert_eq!(homework_points(&p, &t, 5).unwrap(), 27);
        assert_eq!(homework_points(&p, &t, 6).unwrap(), 30);
        assert_eq!(homework_points(&p, &t, 7).unwrap(), 32);
        assert_eq!(homework_points(&p, &t, 8).unwrap(), 37);
        assert_eq!(homework_points(&p, &t, 9).unwrap(), 37);
    }
    
    #[test]
    fn test_homework_points_negative() {
        let p = vec![-8, 2, 4, 5, 9, 6, 3];
        let t = vec![0, 4, 1, 6, 2, 0, 0];
        
        assert_eq!(homework_points(&p, &t, 0).unwrap(), 0);
        assert_eq!(homework_points(&p, &t, 1).unwrap(), 0);
        assert_eq!(homework_points(&p, &t, 2).unwrap(), 6);
        assert_eq!(homework_points(&p, &t, 3).unwrap(), 10);
        assert_eq!(homework_points(&p, &t, 4).unwrap(), 19);
        assert_eq!(homework_points(&p, &t, 5).unwrap(), 22);
        assert_eq!(homework_points(&p, &t, 6).unwrap(), 24);
        assert_eq!(homework_points(&p, &t, 7).unwrap(), 24);
        assert_eq!(homework_points(&p, &t, 8).unwrap(), 29);
        assert_eq!(homework_points(&p, &t, 9).unwrap(), 29);
    }

    #[test]
    fn test_homework_points_mismatched_length() {
        let p = vec![1, 2];
        let t = vec![3];

        assert_eq!(homework_points(&p, &t, 1).is_err(), true);
    }
}
