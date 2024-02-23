mod hashmap;

pub fn add(left: usize, right: usize) -> usize {
    let _h = hashmap::HashMap::<String, String>::new();
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
}
