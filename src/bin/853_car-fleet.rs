//N 辆车沿着一条车道驶向位于 target 英里之外的共同目的地。
//
// 每辆车 i 以恒定的速度 speed[i] （英里/小时），从初始位置 position[i] （英里） 沿车道驶向目的地。
//
// 一辆车永远不会超过前面的另一辆车，但它可以追上去，并与前车以相同的速度紧接着行驶。
//
// 此时，我们会忽略这两辆车之间的距离，也就是说，它们被假定处于相同的位置。
//
// 车队 是一些由行驶在相同位置、具有相同速度的车组成的非空集合。注意，一辆车也可以是一个车队。
//
// 即便一辆车在目的地才赶上了一个车队，它们仍然会被视作是同一个车队。
//
//
//
// 会有多少车队到达目的地?
//
//
//
// 示例：
//
// 输入：target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]
//输出：3
//解释：
//从 10 和 8 开始的车会组成一个车队，它们在 12 处相遇。
//从 0 处开始的车无法追上其它车，所以它自己就是一个车队。
//从 5 和 3 开始的车会组成一个车队，它们在 6 处相遇。
//请注意，在到达目的地之前没有其它车会遇到这些车队，所以答案是 3。
//
//
//
//提示：
//
//
// 0 <= N <= 10 ^ 4
// 0 < target <= 10 ^ 6
// 0 < speed[i] <= 10 ^ 6
// 0 <= position[i] < target
// 所有车的初始位置各不相同。
//
// Related Topics 排序




//leetcode submit region begin(Prohibit modification and deletion)
pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    if position.len() == 0 || speed.len() == 0{
        return 0;
    }

    let mut cars: Vec<(i32, f64)> = Vec::with_capacity(position.len());
    for i in 0..position.len() {
        let time = (target - position[i]) as f64 / speed[i] as f64;
        cars.push((position[i], time));
    }
    //按位置排序
    cars.sort_by(|a, b| i32::cmp(&a.0, &b.0));

    let mut rst = 0;
    let mut i = position.len() - 1;

    while i > 0 {
        if cars[i].1 < cars[i-1].1 {
            //前面的车更快，自成车队
            rst += 1;
        } else {
            //前面的车更慢
            //后面的车追上后减速
            cars[i-1] = cars[i];
        }
        i -= 1;
    }
    return rst + (if i == 0 { 1 } else { 0 });
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn car_fleet_test() {
    let rst = car_fleet(10, vec![6,8], vec![3,2]);
    assert_eq!(rst, 2);
}