use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct City {
        index: i32,
        count: i32,
    }

    impl Ord for City {
        fn cmp(&self, other: &Self) -> Ordering {
            self.count.cmp(&other.count)
        }
    }

    impl PartialOrd for City {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut count: HashMap<i32, i32> = HashMap::new();
    for item in roads {
        count.entry(item[0]).and_modify(|v| *v += 1).or_insert(1);
        count.entry(item[1]).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut heap: BinaryHeap<City> = BinaryHeap::new();
    for (key, value) in count {
        heap.push(City {
            index: key,
            count: value,
        })
    }
    let mut result: i64 = 0;
    let mut current = n;
    while !heap.is_empty() {
        let city = heap.pop().unwrap();
        result += (current * city.count) as i64;
        current -= 1;
    }
    result
}
