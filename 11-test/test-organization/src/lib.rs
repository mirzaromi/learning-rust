pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(input: usize) -> usize {
    internal_adder(input, 2)
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn internal() {
        let result = internal_adder(2,2);
        assert_eq!(result, 4);
    }
}
