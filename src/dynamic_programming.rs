use std::cmp::max;
/// 55.最大子数组和
/// 给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
///
/// 子数组 是数组中的一个连续部分。
///
///
/// 示例 1：
///
/// 输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
/// 输出：6
/// 解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。
/// 示例 2：
///
/// 输入：nums = [1]
/// 输出：1
/// 示例 3：
///
/// 输入：nums = [5,4,-1,7,8]
/// 输出：23
///
/// 提示：
///
/// 1 <= nums.length <= 105
/// -104 <= nums[i] <= 104
///
/// 进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。
/// 
/// https://leetcode.cn/problems/maximum-subarray/solutions/2361770/53-zui-da-zi-shu-zu-he-dong-tai-gui-hua-bvkq9/
/// 解题思路：
/// 动态规划是本题的最优解法，以下按照标准流程解题。
///
/// 常见解法	时间复杂度	空间复杂度
/// 暴力搜索	O(N^2)     O(1)
/// 分治思想	O(Nlog⁡N)	O(log⁡N)
/// 动态规划	O(N)	   O(1)
/// 动态规划解析：
///
/// 状态定义： 设动态规划列表 dp ，dp[i] 代表以元素 nums[i] 为结尾的连续子数组最大和。
///
/// 为何定义最大和 dp[i] 中必须包含元素 nums[i] ：保证 dp[i] 递推到 dp[i+1] 的正确性；如果不包含 nums[i]，递推时则不满足题目的 连续子数组 要求。
/// 转移方程： 若 dp[i−1]≤0 ，说明 dp[i−1] 对 dp[i]产生负贡献，即 dp[i−1]+nums[i] 还不如 nums[i] 本身大。
 ///
/// 初始状态： dp[0]=nums[0]，即以 nums[0] 结尾的连续子数组最大和为 nums[0] 。
///
/// 返回值： 返回 dp 列表中的最大值，代表全局最大值。
/// 状态压缩：
/// 由于 dp[i] 只与 dp[i−1] 和 nums[i] 有关系，因此可以将原数组 nums 用作 dp 列表，即直接在 nums 上修改即可。
/// 由于省去 dp 列表使用的额外空间，因此空间复杂度从 O(N) 降至 O(1) 。
/// <img class="marble" src="https://pic.leetcode-cn.com/77d1aa6a444743d3c8606ac951cd7fc38faf68a62064fd2639df517cd666a4d0-Picture1.png" alt="">
pub fn max_sub_array_dym(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut res = nums[0];
    for i in 1..nums.len() {
        nums[i] += max(nums[i - 1], 0);
        res = max(res, nums[i]);
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array_dym() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_sub_array_dym(nums), 6);
    }
}