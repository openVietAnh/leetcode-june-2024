use std::collections::{HashMap, HashSet};

pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut adjacent: HashMap<i32, Vec<i32>> =
        HashMap::from_iter((0..n).into_iter().map(|i| (i, vec![])));
    for item in edges {
        adjacent.get_mut(&item[0]).unwrap().push(item[1]);
    }
    let mut visited: Vec<bool> = vec![false; n as usize];
    let mut ancestors: HashMap<i32, HashSet<i32>> =
        HashMap::from_iter((0..n).into_iter().map(|i| (i, HashSet::new())));
    let mut stack: Vec<(i32, i32, bool)> = (0..n).into_iter().map(|i| (i, i, false)).collect();
    while !stack.is_empty() {
        let top = stack.len() - 1;
        if stack[top].2 {
            visited[stack[top].1 as usize] = false;
            stack.pop();
        } else {
            let current_node = stack[top].1;
            visited[current_node as usize] = true;
            for item in adjacent.get(&current_node).unwrap() {
                if !visited[*item as usize] {
                    ancestors.get_mut(item).unwrap().insert(stack[top].0);
                    stack.push((stack[top].0, *item, false))
                }
            }
            stack[top].2 = true;
        }
    }
    let mut result: Vec<Vec<i32>> = vec![vec![]; n as usize];
    for (node, ancestors) in ancestors {
        for item in ancestors {
            result[node as usize].push(item);
        }
    }
    for i in 0..n {
        result[i as usize].sort()
    }
    result
}
