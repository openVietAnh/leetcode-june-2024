use std::collections::HashMap;

pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut count: HashMap<i32, usize> = HashMap::new();
    for item in arr1 {
        count.entry(item).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut result = vec![];
    for item in arr2 {
        if count.contains_key(&item) {
            result.append(&mut [item].repeat(*count.get(&item).unwrap()));
            count.remove(&item);
        }
    }
    let mut left_overs = count
        .into_iter()
        .map(|(k, v)| [k].repeat(v))
        .collect::<Vec<Vec<i32>>>()
        .concat();
    left_overs.sort();
    result.append(&mut left_overs);
    result
}
