pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
    hand.sort();
    let mut used: Vec<bool> = vec![false; hand.len()];
    let mut start_ind = 0;
    while start_ind < hand.len() {
        used[start_ind] = true;
        let mut ind = start_ind;
        let mut cur = hand[ind];
        let mut count = 1;
        while count < group_size {
            while ind < hand.len() && used[ind] || hand[ind] == cur {
                ind += 1;
            }
            if ind == hand.len() || hand[ind] > cur + 1 {
                return false;
            }
            used[ind] = true;
            cur = hand[ind];
            count += 1;
        }
        while start_ind < hand.len() && used[start_ind] {
            start_ind += 1;
        }
    }
    true
}
