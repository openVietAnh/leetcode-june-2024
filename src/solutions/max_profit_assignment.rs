pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
    worker.sort();
    println!("{:?}", worker);
    let mut works: Vec<(&i32, &i32)> = profit.iter().zip(difficulty.iter()).collect();
    works.sort_by(|a, b| a.0.cmp(&b.0));
    for i in 0..works.len() {
        print!("{} ", works[i].0);
    }
    println!();
    for i in 0..works.len() {
        print!("{} ", works[i].1);
    }
    println!();
    let mut result = 0;
    let mut count = 0;
    let mut current_work = works.len() - 1;
    let mut used = vec![false; worker.len()];
    while count < worker.len() {
        let required_difficulty = works[current_work].1;
        println!(
            "Current work has {} difficulty and {} profit",
            works[current_work].1, works[current_work].0
        );
        match worker.binary_search(required_difficulty) {
            Ok(mut ind) => {
                while ind > 0 && worker[ind - 1] == worker[ind] {
                    ind -= 1;
                }
                while ind < used.len() && used[ind] {
                    ind += 1;
                }
                if ind < used.len() {
                    println!("Assigned to worker {} with value {}", ind, worker[ind]);
                    result += works[current_work].0;
                    used[ind] = true;
                    count += 1;
                } else {
                    println!("Can't work");
                    if current_work == 0 {
                        break;
                    }
                    current_work -= 1;
                }
            }
            Err(mut ind) => {
                if ind < used.len() {
                    while ind < used.len() && used[ind] {
                        ind += 1;
                    }
                    if ind < used.len() {
                        println!("Assigned to worker {} with value {}", ind, worker[ind]);
                        result += works[current_work].0;
                        used[ind] = true;
                        count += 1;
                    } else {
                        println!("Can't work");
                        if current_work == 0 {
                            break;
                        }
                        current_work -= 1;
                    }
                } else {
                    println!("Can't work");
                    if current_work == 0 {
                        break;
                    }
                    current_work -= 1;
                }
            }
        }
    }
    result
}
