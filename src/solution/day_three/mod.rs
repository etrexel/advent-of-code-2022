use anyhow::anyhow;

pub(crate) mod part_one;
pub(crate) mod part_two;
mod rucksack;

fn compute_priority(input: &char) -> Result<u32, anyhow::Error> {
    if !input.is_ascii_alphabetic() {
        return Err(anyhow!(
            "invalid character for priority computation: {}",
            input
        ));
    }
    let mut val = input.clone() as u32;
    if val >= 65 && val <= 90 {
        val -= 38
    } else if val >= 97 && val <= 122 {
        val -= 96
    }
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_priority() {
        assert_eq!(1, compute_priority(&'a').expect("should return result"));
        assert_eq!(26, compute_priority(&'z').expect("should return result"));
        assert_eq!(27, compute_priority(&'A').expect("should return result"));
        assert_eq!(52, compute_priority(&'Z').expect("should return result"));
    }
}
