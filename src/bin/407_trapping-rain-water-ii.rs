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



//解题思路
//边框向内收缩，从最低节点开始（最小堆）
//节点不可重复计算
//边框高于边框内节点时，可以蓄水



//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::BinaryHeap;
use std::cmp::{Ordering, max};

struct Node{
    row: usize,
    col: usize,
    hei: i32
}

impl Eq for Node{

}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.hei.cmp(&self.hei)
    }
}

impl PartialEq for Node{

    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    if height_map.len() == 0 {
        return 0;
    }

    if height_map[0].len() == 0 {
        return 0;
    }

    let row_cnt = height_map.len();
    let col_cnt = height_map[0].len();
    let mut visited:Vec<Vec<bool>> =  Vec::with_capacity(row_cnt);

    //init visited
    for _ in 0..row_cnt {
        let mut row = Vec::with_capacity(col_cnt);
        for _ in 0..col_cnt{
            row.push(false);
        }
        visited.push(row);
    }


    let mut heap:BinaryHeap<Node> = BinaryHeap::with_capacity(row_cnt * col_cnt );

    //将竖边框放入堆
    for r in 0..row_cnt {
        heap.push(Node{
            row: r,
            col: 0,
            hei: height_map[r][0]
        });
        heap.push(Node{
            row: r,
            col: col_cnt - 1,
            hei: height_map[r][col_cnt-1]
        });
        visited[r][0] = true;
        visited[r][col_cnt-1] = true;
    }

    //将横边框放入堆
    for c in 0..col_cnt {
        heap.push(Node{
            row: 0,
            col: c,
            hei: height_map[0][c]
        });
        heap.push(Node{
            row: row_cnt-1,
            col: c,
            hei: height_map[row_cnt-1][c]
        });
        visited[0][c] = true;
        visited[row_cnt-1][c] = true;
    }

    let mut curr_min_bound = std::i32::MIN;
    let mut total = 0;
    while heap.len() > 0 {
        //取出边框中高度最低的节点
        let node = heap.pop().unwrap();
        curr_min_bound = max(curr_min_bound, node.hei);

        //下边框，向上收缩
        if node.row > 0 && !visited[node.row-1][node.col]{
            heap.push(Node{
                row: node.row-1,
                col: node.col,
                hei: height_map[node.row-1][node.col]
            });
            visited[node.row-1][node.col] = true;

            let target_height = height_map[node.row-1][node.col];
            if target_height < curr_min_bound{
                total += curr_min_bound - target_height;
            }
        }

        //上边框，向下收缩
        if node.row < row_cnt-1 && !visited[node.row+1][node.col]{
            heap.push(Node{
                row: node.row+1,
                col: node.col,
                hei: height_map[node.row+1][node.col]
            });
            visited[node.row+1][node.col] = true;

            let target_height = height_map[node.row+1][node.col];
            if target_height < curr_min_bound{
                total += curr_min_bound - target_height;
            }
        }

        //左边框，向右收缩
        if node.col < col_cnt-1 && !visited[node.row][node.col+1]{
            heap.push(Node{
                row: node.row,
                col: node.col+1,
                hei: height_map[node.row][node.col+1]
            });
            visited[node.row][node.col+1] = true;

            let target_height = height_map[node.row][node.col+1];
            if target_height < curr_min_bound{
                total += curr_min_bound - target_height;
            }
        }

        //右边框，向左收缩
        if node.col > 0 && !visited[node.row][node.col-1]{
            heap.push(Node{
                row: node.row,
                col: node.col-1,
                hei: height_map[node.row][node.col-1]
            });
            visited[node.row][node.col-1] = true;

            let target_height = height_map[node.row][node.col-1];
            if target_height < curr_min_bound{
                total += curr_min_bound - target_height;
            }
        }
    }

    return total;
}
//leetcode submit region end(Prohibit modification and deletion)


#[test]
fn trap_rain_water_test() {
    let rst = trap_rain_water(vec![
        vec![1,4,3,1,3,2],
        vec![3,2,1,3,2,4],
        vec![2,3,3,2,3,1]
    ]);
    assert_eq!(rst, 4);
}