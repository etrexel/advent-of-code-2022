use crate::solution::day_three::compute_priority;
use anyhow::anyhow;

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let groups = create_groups(input)?;
    let commons = find_group_commons(&groups)?;
    let priority = sum_priorities(&commons)?;
    Ok(priority.to_string())
}

fn create_groups(input: &str) -> Result<Vec<Vec<String>>, anyhow::Error> {
    let mut groups = Vec::new();
    let lines: Vec<&str> = input.split("\n").collect();
    if lines.len() % 3 != 0 {
        return Err(anyhow!("must be at least three in each group"));
    }
    for i in (0..lines.len()).step_by(3) {
        groups.push(vec![
            lines[i].to_string(),
            lines[i + 1].to_string(),
            lines[i + 2].to_string(),
        ]);
    }
    Ok(groups)
}

fn find_group_commons(groups: &Vec<Vec<String>>) -> Result<Vec<char>, anyhow::Error> {
    let mut commons = Vec::new();
    for group in groups {
        if let Some(common) = find_common(group)? {
            commons.push(common)
        }
    }
    Ok(commons)
}

fn sum_priorities(commons: &Vec<char>) -> Result<u32, anyhow::Error> {
    let mut total = 0;
    for item in commons {
        total += compute_priority(item)?;
    }
    Ok(total)
}

fn find_common(group: &Vec<String>) -> Result<Option<char>, anyhow::Error> {
    if group.len() != 3 {
        return Err(anyhow!("groups can only have three members"));
    }
    for item in group[0].chars() {
        if group[1].find(item).is_some() && group[2].find(item).is_some() {
            return Ok(Some(item));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!("70", solve(&input).expect("should return result"));
    }

    #[test]
    fn test_find_common() {
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
        ];
        assert_eq!(
            Some('r'),
            find_common(&input).expect("should return result")
        )
    }
}
