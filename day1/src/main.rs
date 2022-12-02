fn find_highest_calorie_count_basic(data: String) -> Option<u32> {
    let mut elve_calorie_counts = vec![];
    let mut current_calorie_count = 0;

    for line in data.lines() {
        if line.trim().len() == 0 {
            elve_calorie_counts.push(current_calorie_count);
            current_calorie_count = 0;
        } else {
            let line_count = line.parse::<u32>().unwrap();

            current_calorie_count += line_count;
        }
    }

    if current_calorie_count != 0 {
        elve_calorie_counts.push(current_calorie_count);
    }

    elve_calorie_counts.sort();

    elve_calorie_counts.last().copied()
}

fn find_highest_calorie_count_map(data: String) -> Option<u32> {
    let parsed_line_items = data
        .lines()
        .map(|line_item| line_item.parse::<u32>().ok())
        .collect::<Vec<Option<u32>>>();

    let grouped_line_items = parsed_line_items
        .split(|item| item.is_none())
        .map(|group| group.to_vec().iter().map(|item| item.unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    let mut summed_groups = grouped_line_items
        .into_iter()
        .map(|elf_group| elf_group.into_iter().sum())
        .collect::<Vec<u32>>();

    summed_groups.sort_by(|a, b| b.cmp(a));

    summed_groups
        .into_iter()
        .take(3)
        .fold(0, |i, acc| acc + i)
        .into()
}

fn run_with_elapsed_time<F>(func: F, data: String, func_name: &str)
where
    F: Fn(String) -> Option<u32>,
{
    let start = std::time::Instant::now();

    let func_result = func(data);

    let end = start.elapsed();

    println!(
        "Finished running {} with result: {}\ttook {}ns",
        func_name,
        func_result.unwrap(),
        end.as_nanos()
    );
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    run_with_elapsed_time(find_highest_calorie_count_basic, data.clone(), "basic");
    run_with_elapsed_time(find_highest_calorie_count_map, data.clone(), "mapreduce");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_input_basic() {
        let data = std::fs::read_to_string("example.txt").unwrap();

        let highest_calorie_count = find_highest_calorie_count_basic(data);

        assert_eq!(Some(24_000), highest_calorie_count)
    }

    #[test]
    fn test_given_input_fp() {
        let data = std::fs::read_to_string("example.txt").unwrap();

        let highest_calorie_count = find_highest_calorie_count_map(data);

        assert_eq!(Some(24_000), highest_calorie_count)
    }
}
