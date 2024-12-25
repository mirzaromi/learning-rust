fn main() {
    println!("Hello, world!");
}


fn prints_and_returns_10(input: i32) -> i32 {
    println!("I got the value {input}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }
    
    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5); 
    }
}

pub fn add_two(input: usize) -> usize {
    input + 2
}

#[cfg(test)]
mod tests_multiple {
    use super::*;
    
    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }
    
    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
    
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}