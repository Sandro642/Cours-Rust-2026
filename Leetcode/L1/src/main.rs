use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    //let nums = vec![2,7,11,15];
    let nums = vec![3,3];
    let target = 6;

    let result = two_sum(nums, target);
    println!("{:?}", result);
}
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &cur) in nums.iter().enumerate() {
            let x = target - cur;

            if let Some(&index) = map.get(&x) {
                return vec![index as i32, i as i32]
            }
            map.insert(cur, i);
        }
        vec![]
    }