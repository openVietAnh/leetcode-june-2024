use std::{
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque},
};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Project {
    profit: i32,
    capital: i32,
}

impl Ord for Project {
    fn cmp(&self, other: &Self) -> Ordering {
        self.profit
            .cmp(&other.profit)
            .then_with(|| other.capital.cmp(&self.capital))
    }
}

impl PartialOrd for Project {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut result = w;
    let mut heap: BinaryHeap<Project> = BinaryHeap::new();
    for i in 0..profits.len() {
        heap.push(Project {
            profit: profits[i],
            capital: capital[i],
        })
    }
    let mut count = 0;
    while count < k {
        let mut queue: VecDeque<Project> = VecDeque::new();
        let mut chose: bool = false;
        while !heap.is_empty() {
            let current_item = heap.pop().unwrap();
            if result >= current_item.capital {
                count += 1;
                result += current_item.profit;
                chose = true;
                break;
            } else {
                queue.push_back(current_item);
            }
        }
        if !chose {
            break;
        } else {
            while !queue.is_empty() {
                heap.push(queue.pop_front().unwrap());
            }
        }
    }
    result
}
