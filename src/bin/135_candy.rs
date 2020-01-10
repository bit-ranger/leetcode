//老师想给孩子们分发糖果，有 N 个孩子站成了一条直线，老师会根据每个孩子的表现，预先给他们评分。
//
// 你需要按照以下要求，帮助老师给这些孩子分发糖果：
//
//
// 每个孩子至少分配到 1 个糖果。
// 相邻的孩子中，评分高的孩子必须获得更多的糖果。
//
//
// 那么这样下来，老师至少需要准备多少颗糖果呢？
//
// 示例 1:
//
// 输入: [1,0,2]
//输出: 5
//解释: 你可以分别给这三个孩子分发 2、1、2 颗糖果。
//
//
// 示例 2:
//
// 输入: [1,2,2]
//输出: 4
//解释: 你可以分别给这三个孩子分发 1、2、1 颗糖果。
//     第三个孩子只得到 1 颗糖果，这已满足上述两个条件。
// Related Topics 贪心算法






//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::max;

fn count(n: i32) -> i32 {
    return (n * (n + 1)) / 2;
}

pub fn candy(ratings: Vec<i32>) -> i32 {
    if ratings.len() == 0 {
        return 0;
    }
    let mut candies = 0;
    let mut up = 0;
    let mut down = 0;
    let mut old_slope = 0;
    for i in 1..ratings.len() {
        // 1上坡，-1下坡，0平坡
        let new_slope = if ratings[i] > ratings[i-1] {1} else if ratings[i] < ratings[i-1] {-1} else {0};
        // 坡峰->平坡 || 坡谷->上坡/平坡
        if (old_slope > 0 && new_slope == 0) || (old_slope < 0 && new_slope >= 0) {
            candies += count(up) + count(down) + max(up, down);
            up = 0;
            down = 0;
        }

        if new_slope > 0 {
            up+=1;
        }

        if new_slope < 0 {
            down+=1;
        }

        if new_slope == 0 {
            candies+=1;
        }

        old_slope = new_slope;
    }
    candies += count(up) + count(down) + max(up, down) + 1;
    return candies;
}
//leetcode submit region end(Prohibit modification and deletion)


#[test]
fn candy_test() {
    let rst = candy(vec![1,2,2]);
    assert_eq!(rst, 4);

    let rst = candy(vec![1,0,2]);
    assert_eq!(rst, 5);
}