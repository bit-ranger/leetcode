//给定一个 m x n 的矩阵，其中的值均为正整数，代表二维高度图每个单元的高度，请计算图中形状最多能接多少体积的雨水。
//
//
//
// 说明:
//
// m 和 n 都是小于110的整数。每一个单位的高度都大于 0 且小于 20000。
//
//
//
// 示例：
//
// 给出如下 3x6 的高度图:
//[
//  [1,4,3,1,3,2],
//  [3,2,1,3,2,4],
//  [2,3,3,2,3,1]
//]
//
//返回 4。
//
//
//
//
// 如上图所示，这是下雨前的高度图[[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]] 的状态。
//
//
//
//
//
// 下雨后，雨水将会被存储在这些方块中。总的接雨水量是4。
// Related Topics 堆 广度优先搜索




use std::collections::BinaryHeap;
use std::cmp::Ordering;

//leetcode submit region begin(Prohibit modification and deletion)
struct Node{
    row: usize,
    col: usize,
    hei: i32
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.hei.cmp(&other.hei)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let row_cnt = height_map.len();
        let col_cnt = height_map[0].len();
        let mut visited:Vec<Vec<i32>> =  Vec::new();

        let mut heap:BinaryHeap<Node> = BinaryHeap::with_capacity(row_cnt * col_cnt );

        //将竖边框放入堆
        for r in 0..row_cnt {
            heap.add(Node{
                row: r,
                col: 0,
                hei: height_map[r][0]
            });
            heap.add(Node{
                row: r,
                col: col_cnt - 1,
                hei: height_map[r][col_cnt-1]
            });
            visited[r][0] = 1;
            visited[r][col_cnt-1] = 1;
        }

        //将横边框放入堆
        for c in 0..col_cnt {
            heap.add(Node{
                row: 0,
                col: c,
                hei: height_map[0][c]
            });
            heap.add(Node{
                row: row_cnt-1,
                col: c,
                hei: height_map[row_cnt-1][c]
            });
            visited[0][c] = 1;
            visited[row_cnt-1][c] = 1;
        }




    }
}
//leetcode submit region end(Prohibit modification and deletion)
