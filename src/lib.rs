use hashmap::MyHashmap;

mod hashmap;

pub fn hashmap() {
    let mut h = MyHashmap::new();
    h.insert("1", "one");
    h.remove("2");
    let x = h.get("1");
    println!("{:?}", x);
    h.clear();
}
