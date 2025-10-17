use hashmap::MyHashmap;

mod hashmap;
mod tree;

pub fn hashmap() {
    let mut h = MyHashmap::new();
    h.insert("1", "one");
    h.remove("2");
    let x = h.get("1");
    println!("{:?}", x);
    h.clear();
}
#[derive(Debug)]
struct Mine {}
pub fn try_it_out() {
    let b = Box::new(Mine {});
    let x = b.as_ref();
    let z = b.as_ref().to_owned();
    println!("{:?}", b);
    println!("{:?}", x);
    println!("{:?}", z);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        try_it_out();
    }
}
