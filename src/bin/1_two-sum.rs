//给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。 
//
// 你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。 
//
// 示例: 
//
// 给定 nums = [2, 7, 11, 15], target = 9
//
//因为 nums[0] + nums[1] = 2 + 7 = 9
//所以返回 [0, 1]
// 
// Related Topics 数组 哈希表



//leetcode submit region begin(Prohibit modification and deletion)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map:HashMap<i32, i32> =  HashMap::new();
    for (i, num) in nums.into_iter().enumerate(){
        let complement = target-num;
        if map.contains_key(&complement) {
            return vec![map.get(&complement).unwrap().clone(), i as i32];
        }
        map.insert(num, i as i32);
    }
    panic!("No two sum solution")
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rst = two_sum(vec![2, 4, 6, 3], 7);
        assert_eq!(vec![1,3],rst);
    }
}