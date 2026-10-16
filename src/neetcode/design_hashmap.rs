#![allow(unused)]
struct MyHashMap {
    data: Vec<i32>,
}

impl MyHashMap {
    pub fn new() -> Self {
        Self {
            data: vec![-1; 1_000_001],
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.data[key as usize] = value;
    }

    pub fn get(&self, key: i32) -> i32 {
        self.data[key as usize]
    }

    pub fn remove(&mut self, key: i32) {
        self.data[key as usize] = -1;
    }
}

pub fn main() {
    let mut map = MyHashMap::new();
    println!("{:?}", map.put(1000000, 0));
    println!("{:?}", map.put(2, 2));
    println!("{:?}", map.get(1_000_000));
    println!("{:?}", map.get(3));
    println!("{:?}", map.put(2, 1));
    println!("{:?}", map.get(2));
    println!("{:?}", map.remove(2));
    println!("{:?}", map.get(2));
}
