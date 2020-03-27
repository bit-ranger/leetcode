//给出一些不同颜色的盒子，盒子的颜色由数字表示，即不同的数字表示不同的颜色。
//你将经过若干轮操作去去掉盒子，直到所有的盒子都去掉为止。每一轮你可以移除具有相同颜色的连续 k 个盒子（k >= 1），这样一轮之后你将得到 k*k 个积分
//。
//当你将所有盒子都去掉之后，求你能获得的最大积分和。
//
// 示例 1：
//输入:
//
//
//[1, 3, 2, 2, 2, 3, 4, 3, 1]
//
//
// 输出:
//
//
//23
//
//
// 解释:
//
//
//[1, 3, 2, 2, 2, 3, 4, 3, 1]
//----> [1, 3, 3, 4, 3, 1] (3*3=9 分)
//----> [1, 3, 3, 3, 1] (1*1=1 分)
//----> [1, 1] (3*3=9 分)
//----> [] (2*2=4 分)
//
//
//
//
// 提示：盒子的总数 n 不会超过 100。
// Related Topics 深度优先搜索 动态规划


use std::collections::HashMap;
use std::cmp::max;

//leetcode submit region begin(Prohibit modification and deletion)

fn calculate_points(boxes: Vec<i32>, l:usize, r:usize, n:usize) -> usize{
    //只剩一个
    if l == r {
        return (n+1)*(n+1);
    }

    //left与邻居相同, 与邻居一起算
    if boxes[l] == boxes[l+1] {
        return calculate_points(boxes, l+1, r, n+1);
    }

    //算出本序列得分
    let mut point = (n+1)*(n+1) + calculate_points(boxes, l+1, r, 0);

    //l向后寻找相同的, l与后一个必定不同
    for l_s in l+2..r{
        if boxes[l_s] == boxes[l]{
            point = max(point,
            calculate_points(boxes, l+1, l_s, 0) + calculate_points(boxes, l_s, r, n+1)
            )
        }
    }

}

pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
    return calculate_points(boxes, 0, boxes.len()-1, 0) as i32;
}
//leetcode submit region end(Prohibit modification and deletion)
