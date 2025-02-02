//! Hashmap?
//! Key: Name
//! Value: (time, emotion)

use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(u32, String)>>
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: u32) {
        self.map.entry(key)
            .and_modify(|v| {
                let time: Vec<u32> = v
                    .iter()
                    .map(|(t, _v)| *t)
                    .collect();

                //find timestamp
                let mut l = 0; 
                let mut r = time.len();
                while l < r {
                    let m = l + (r - l) / 2;
                    if time[m] >= timestamp {
                        r = m;
                    } else {
                        l = m + 1;
                    }
                }
                let idx = l;
                if l >= time.len() {
                    v.push((timestamp, value.clone()));
                } else {
                    v.insert(idx, (timestamp, value.clone()));
                }
            })
            .or_insert(vec![(timestamp, value.clone())]);
    }

    fn get(&self, key: String, timestamp: u32) -> String {
        if let None = self.map.get(&key) {
            return "".to_string();
        } else {
            let v = self.map.get(&key).unwrap();
            let mut l = 0;
            let mut r = v.len();

            while l < r {
                let t = l + (r - l) / 2;
                if t > timestamp as usize {
                    r = t;
                } else {
                    l = t + 1;
                }
            }
            if l > 0 {return v[l - 1].1.clone();} else {return "".to_string();}
        }
    }
}


fn main() {
    let mut time_map = TimeMap::new();
    time_map.set("alice".into(), "happy".into(), 1);
    println!("{:?}", time_map.map);
    println!("{}", time_map.get("alice".into(), 1));
    println!("{}", time_map.get("alice".into(), 2));
    time_map.set("alice".into(), "sad".into(), 3);
    println!("{:?}", time_map.map);
    println!("{}", time_map.get("alice".into(), 3));
}
