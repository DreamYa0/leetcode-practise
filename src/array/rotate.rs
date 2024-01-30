/// 旋转数组
/// 给定一个整数数组 nums，将数组中的元素向右轮转 k 个位置，其中 k 是非负数。
/// 示例 1:
/// 输入: nums = [1,2,3,4,5,6,7], k = 3
/// 输出: [5,6,7,1,2,3,4]
/// 解释:
/// 向右轮转 1 步: [7,1,2,3,4,5,6]
/// 向右轮转 2 步: [6,7,1,2,3,4,5]
/// 向右轮转 3 步: [5,6,7,1,2,3,4]
/// 示例 2:
/// 输入：nums = [-1,-100,3,99], k = 2
/// 输出：[3,99,-1,-100]
/// 解释:
/// 向右轮转 1 步: [99,-1,-100,3]
/// 向右轮转 2 步: [3,99,-1,-100]
/// 提示：
/// 1 <= nums.length <= 105
/// -231 <= nums[i] <= 231 - 1
/// 0 <= k <= 105
/// 进阶：
/// 尽可能想出更多的解决方案，至少有 三种 不同的方法可以解决这个问题。
/// 你可以使用空间复杂度为 O(1) 的 原地 算法解决这个问题吗？
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    // 核心点 k % len 因为k有可能超过数组长度
    let k = k as usize % len;
    reverse(nums, 0, len);
    reverse(nums, 0, k as usize);
    reverse(nums, k as usize, len);
}

/// 反转数组 , 左闭，右闭 区间
fn reverse(nums: &mut Vec<i32>, mut start: usize, mut end: usize) {
    while start < end {
        let temp = nums[start];
        nums[start] = nums[end - 1];
        nums[end - 1] = temp;
        start += 1;
        end -= 1;
    }
}

/// 旋转函数
/// 给定一个长度为 n 的整数数组 nums 。
///
/// 假设 arrk 是数组 nums 顺时针旋转 k 个位置后的数组，我们定义 nums 的 旋转函数  F 为：
///
/// F(k) = 0 * arrk[0] + 1 * arrk[1] + ... + (n - 1) * arrk[n - 1]
/// 返回 F(0), F(1), ..., F(n-1)中的最大值 。
///
/// 生成的测试用例让答案符合 32 位 整数。
///
/// 示例 1:
///
/// 输入: nums = [4,3,2,6]
/// 输出: 26
/// 解释:
/// F(0) = (0 * 4) + (1 * 3) + (2 * 2) + (3 * 6) = 0 + 3 + 4 + 18 = 25
/// F(1) = (0 * 6) + (1 * 4) + (2 * 3) + (3 * 2) = 0 + 4 + 6 + 6 = 16
/// F(2) = (0 * 2) + (1 * 6) + (2 * 4) + (3 * 3) = 0 + 6 + 8 + 9 = 23
/// F(3) = (0 * 3) + (1 * 2) + (2 * 6) + (3 * 4) = 0 + 2 + 12 + 12 = 26
/// 所以 F(0), F(1), F(2), F(3) 中的最大值是 F(3) = 26 。
/// 示例 2:
///
/// 输入: nums = [100]
/// 输出: 0
///
/// 提示:
///
/// n == nums.length
/// 1 <= n <= 105
/// -100 <= nums[i] <= 100
///
/// 解题思路
/// 假设数组为[a, b, c, d, e]，长度为n，数组和为sum，可以发现旋转函数如下：
///
/// F(0) = 0 * a + 1 * b + 2 * c + 3 * d + 4 * e
/// F(1) = 1 * a + 2 * b + 3 * c + 4 * d + 0 * e
/// F(2) = 2 * a + 3 * b + 4 * c + 0 * d + 1 * e
/// ...
/// F(1) - F(0) = a + b + c + d + e - 5 * e
/// F(2) - F(1) = a + b + c + d + e - 5 * d
/// ...
/// 所以 F(k) = F(k - 1) + sum - n * nums[n - k]，遍历过程每次取最大值即可。
/// 当k = 0时，数组不回发生顺时针旋转，所以F(0) = 0 * a + 1 * b + 2 * c + 3 * d + 4 * e，所以可以先计算出F(0)的值。
/// 向右旋转一次，就相当于把当前结果加上整个数组的和，再减去数组大小乘以当前最后一位
pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
    // 数组和
    let sum: i32 = nums.iter().sum();
    // 数组长度
    let n = nums.len();
    // 计算F(0)
    let mut f = nums
        .iter()
        .enumerate()
        // 这个闭包接受两个参数，第一个参数是累加器的当前值，第二个参数是一个元组，这个元组包含一个元素的索引和这个元素的值。
        // 闭包的作用是将元素的值与其索引的乘积加到累加器中。
        .fold(0, |acc, (i, x)| acc + (i as i32) * x);
    // 记录最大的结果
    let mut res = f;
    for i in 1..n {
        // 计算F(1) 到 F(n - 1)
        f = f + sum - (n as i32) * nums[n - i];
        res = res.max(f)
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        println!("{:?}", nums)
    }
}
