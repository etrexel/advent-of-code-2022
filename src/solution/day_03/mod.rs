use anyhow::anyhow;

pub(crate) mod part_1;
pub(crate) mod part_2;
mod rucksack;

fn compute_priority(input: &char) -> Result<u32, anyhow::Error> {
    if !input.is_ascii_alphabetic() {
        return Err(anyhow!(
            "invalid character for priority computation: {}",
            input
        ));
    }
    let mut val = *input as u32;
    if (65..=90).contains(&val) {
        val -= 38
    } else if (97..=122).contains(&val) {
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
