fn get_digits(number: usize) -> Vec<u32> {
    let mut digits = Vec::new();

    for character in number.to_string().chars() {
        let d = character.to_digit(10);
        match d {
            Some(number) => digits.push(number),
            None => (),
        }
    }

    digits
}

fn get_parity_bit(digits: Vec<u32>) -> u32 {
    let mut loop_mult: u32 = 0;

    for factor in 1..=digits.len() {
        if factor % 2 == 0 {
            loop_mult += digits[factor - 1] * 3
        } else {
            loop_mult += digits[factor - 1] * 1
        }
    }

    let mod_result: u32 = loop_mult % 10;
    if mod_result != 0 {
        return 10 - mod_result;
    } else {
        return mod_result;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        let d = 123;
        assert_eq!(vec![1, 2, 3], get_digits(d))
    }

    #[test]
    fn test_bit() {
        let isbn: usize = 978186197271;
        let digits = get_digits(isbn);
        assert_eq!(2, get_parity_bit(digits))
    }
}
