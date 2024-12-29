fn main() {
    let v1 = vec![1,2,3,];
    
    let v1_iter = v1.iter();
    
    for val in v1_iter {
        println!("got: {val}");
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1,2,3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_demonstration_into_iter() {
        let v1 = vec![1,2,3];

        let mut v1_iter = v1.into_iter();

        assert_eq!(v1_iter.next(), Some(1));
        assert_eq!(v1_iter.next(), Some(2));
        assert_eq!(v1_iter.next(), Some(3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_demonstration_iter_mut() {
        let mut v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter_mut();

        assert_eq!(v1_iter.next(), Some(&mut 1));
        assert_eq!(v1_iter.next(), Some(&mut 2));
        assert_eq!(v1_iter.next(), Some(&mut 3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();
        
        let total:i32 = v1_iter.sum();
        
        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let v1_map = v1_iter.map(|x| x + 1);

        let v1_new: Vec<i32> = v1_map.collect();
        assert_eq!(v1_new, vec![2,3,4]);
    }
    
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 13,
                style: String::from("sandal")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            }
        ];
        
        let in_my_size = shoes_in_size(shoes, 10);
        
        assert_eq!(in_my_size, 
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
             Shoe {
                 size: 10,
                 style: String::from("boot")
             }]
        )
    }
}