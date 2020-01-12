//给定一个整数数组 nums，其中恰好有两个元素只出现一次，其余所有元素均出现两次。 找出只出现一次的那两个元素。
//
// 示例 :
//
// 输入: [1,2,1,3,2,5]
//输出: [3,5]
//
// 注意：
//
//
// 结果输出的顺序并不重要，对于上面的例子， [5, 3] 也是正确答案。
// 你的算法应该具有线性时间复杂度。你能否仅使用常数空间复杂度来实现？
//
// Related Topics 位运算




//leetcode submit region begin(Prohibit modification and deletion)
pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    // difference between two numbers (x and y) which were seen only once
    let mut bitmask = 0;
    for num in nums.iter(){
        bitmask ^= num.clone();
    }

    // rightmost 1-bit diff between x and y
    let diff = bitmask & (-bitmask);

    let mut x = 0;
    // bitmask which will contain only x
    for num in nums.iter() {
        if (num & diff) != 0{
            x ^= num.clone();
        }
    }

    return vec!{x, bitmask^x};
}
//leetcode submit region end(Prohibit modification and deletion)



#[test]
fn single_number_test() {
    let rst = single_number(vec![1,2,1,3,2,5]);
    assert_eq!(rst, vec![3,5]);
}