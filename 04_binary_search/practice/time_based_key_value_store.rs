use std::collections::HashMap;
struct TimeMap {
    map: HashMap<String, Vec<(usize, String)>>
}

impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    fn set(&mut self, key: &str, value: &str, timestamp: usize) {
        self.map
            .entry(key.to_string())
            .and_modify(|v| {
                v.push((timestamp, value.to_string()))
            })
            .or_insert(vec![(timestamp, value.to_string())]);
    }

    fn get(&self, key: &str, timestamp: usize) -> String {

        if let None = self.map.get(&key.to_string()) {
            return String::from("");
        }


        let v = self.map.get(&key.to_string()).unwrap();
        let mut l = 0;
        if timestamp < v[l].0 {
            return String::from("");
        }
        let mut r = v.len();

        //upper bound
        while l < r {
            let m = l + (r - l) / 2;
            if v[m].0 > timestamp {
                r = m;
            } else {
                l = m + 1;
            }
        }

        return v[l - 1].1.clone();
    }
}

fn main() {
    let mut time_map = TimeMap::new();
    time_map.set("alice", "happy", 1);
    println!("{:?}", time_map.map);
    println!("{}", time_map.get("alice", 1)); //happy
    println!("{}", time_map.get("alice", 2)); //happy
    println!("{}", time_map.get("alice", 0)); //
    println!("{}", time_map.get("sally", 5)); //

    time_map.set("alice", "sad", 3);
    println!("{:?}", time_map.map);
    println!("{}", time_map.get("alice", 3)); //sad

}