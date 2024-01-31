/// LCR 016. 无重复字符的最长子串
///
/// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长连续子字符串 的长度。
///
/// 示例 1:
///
/// 输入: s = "abcabcbb"
/// 输出: 3
/// 解释: 因为无重复字符的最长子字符串是 "abc"，所以其长度为 3。
/// 示例 2:
///
/// 输入: s = "bbbbb"
/// 输出: 1
/// 解释: 因为无重复字符的最长子字符串是 "b"，所以其长度为 1。
/// 示例 3:
///
/// 输入: s = "pwwkew"
/// 输出: 3
/// 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
///      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
/// 示例 4:
///
/// 输入: s = ""
/// 输出: 0
///
///
/// 提示：
///
/// 0 <= s.length <= 5 * 104
/// s 由英文字母、数字、符号和空格组成
///
///
/// 注意：本题与主站 3 题相同： https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
/// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
///
/// 复杂度分析
///
/// 时间复杂度：O(n)，其中 n 为 s 的长度。注意 left 至多增加 n 次，所以整个二重循环至多循环 O(n) 次。
///
/// 空间复杂度：O(∣Σ∣)，其中 ∣Σ∣ 为字符集合的大小，本题中字符均为 ASCII 字符，所以 ∣Σ∣≤128
pub fn length_of_longest_substring(st: String) -> i32 {
    let s = st.as_bytes();
    // 最长子串长度
    let mut ans = 0;
    // 起点指针
    let mut left = 0;
    // 用来标记窗口内是否存在重复元素，也可以用 HashSet，这里为了效率用的 Vec
    let mut window = vec![false; 128];
    for (right, &c) in s.iter().enumerate() {
        let c = c as usize;
        // 缩小窗口的条件是，窗口内存在重复元素
        while window[c] {
            // 把起点指针右移一位，并且把起点指针所在的元素从窗口中移除
            window[s[left] as usize] = false;
            left += 1;
        }

        // 往窗口中添加元素
        window[c] = true;
        // 更新窗口长度最大值
        ans = ans.max(right - left + 1);
    }

    ans as i32
}

/// 209. 长度最小的子数组
/// 给定一个含有 n 个正整数的数组和一个正整数 target 。
///
/// 找出该数组中满足其总和大于等于 target 的长度最小的 连续子数组 [numsl, numsl+1, ..., numsr-1, numsr] ，并返回其长度。如果不存在符合条件的子数组，返回 0 。
///
/// 示例 1：
///
/// 输入：target = 7, nums = [2,3,1,2,4,3]
/// 输出：2
/// 解释：子数组 [4,3] 是该条件下的长度最小的子数组。
/// 示例 2：
///
/// 输入：target = 4, nums = [1,4,4]
/// 输出：1
/// 示例 3：
///
/// 输入：target = 11, nums = [1,1,1,1,1,1,1,1]
/// 输出：0
///
///
/// 提示：
///
/// 1 <= target <= 109
/// 1 <= nums.length <= 105
/// 1 <= nums[i] <= 105
///
///
/// 进阶：
///
/// 如果你已经实现 O(n) 时间复杂度的解法, 请尝试设计一个 O(n log(n)) 时间复杂度的解法。
///
/// 滑动窗口
///
/// 接下来就开始介绍数组操作中另一个重要的方法：滑动窗口。
///
/// 所谓滑动窗口，就是不断的调节子序列的起始位置和终止位置，从而得出我们要想的结果。
///
/// 在暴力解法中，是一个for循环滑动窗口的起始位置，一个for循环为滑动窗口的终止位置，用两个for循环 完成了一个不断搜索区间的过程。
///
/// 那么滑动窗口如何用一个for循环来完成这个操作呢。
///
/// 首先要思考 如果用一个for循环，那么应该表示 滑动窗口的起始位置，还是终止位置。
///
/// 如果只用一个for循环来表示 滑动窗口的起始位置，那么如何遍历剩下的终止位置？
///
/// 此时难免再次陷入 暴力解法的怪圈。
///
/// 所以 只用一个for循环，那么这个循环的索引，一定是表示 滑动窗口的终止位置。
///
/// 那么问题来了， 滑动窗口的起始位置如何移动呢？
///
/// 这里还是以题目中的示例来举例，s=7， 数组是 2，3，1，2，4，3，来看一下查找的过程：
/// <img class="marble" src="https://code-thinking.cdn.bcebos.com/gifs/209.%E9%95%BF%E5%BA%A6%E6%9C%80%E5%B0%8F%E7%9A%84%E5%AD%90%E6%95%B0%E7%BB%84.gif" alt="">
///
/// 最后找到 4，3 是最短距离。
///
/// 其实从动画中可以发现滑动窗口也可以理解为双指针法的一种！只不过这种解法更像是一个窗口的移动，所以叫做滑动窗口更适合一些。
///
/// 在本题中实现滑动窗口，主要确定如下三点：
///
/// 窗口内是什么？
/// 如何移动窗口的起始位置？
/// 如何移动窗口的结束位置？
/// 窗口就是 满足其和 ≥ s 的长度最小的 连续 子数组。
///
/// 窗口的起始位置如何移动：如果当前窗口的值大于s了，窗口就要向前移动了（也就是该缩小了）。
///
/// 窗口的结束位置如何移动：窗口的结束位置就是遍历数组的指针，也就是for循环里的索引。
///
/// 解题的关键在于 窗口的起始位置如何移动，如图所示：
/// <img class="marble" src="https://code-thinking-1253855093.file.myqcloud.com/pics/20210312160441942.png" alt="">
///
/// 可以发现滑动窗口的精妙之处在于根据当前子序列和大小的情况，不断调节子序列的起始位置。从而将O(n^2)暴力解法降为O(n)。
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    // 定义起始指针
    let mut left = 0;
    // 定义和
    let mut sum = 0;
    // 定义结果
    let mut res = std::i32::MAX;
    for right in 0..nums.len() {
        // 累加
        sum += nums[right];
        // 循环的条件，当和大于等于目标值时，就要开始缩小窗口了
        while sum >= target {
            // (right - left + 1) 子序列的长度
            res = res.min((right - left + 1) as i32);
            // 调整窗口
            sum -= nums[left];
            // 左指针右移
            left += 1;
        }
    }
    // 如果res没有被赋值，就返回0
    if res == std::i32::MAX {
        0
    } else {
        res
    }
}

/// 485. 最大连续 1 的个数
/// 给定一个二进制数组 nums ， 计算其中最大连续 1 的个数。
///
///
/// 示例 1：
///
/// 输入：nums = [1,1,0,1,1,1]
/// 输出：3
/// 解释：开头的两位和最后的三位都是连续 1 ，所以最大连续 1 的个数是 3.
/// 示例 2:
///
/// 输入：nums = [1,0,1,1,0,1]
/// 输出：2
///
/// 提示：
///
/// 1 <= nums.length <= 105
/// nums[i] 不是 0 就是 1.
///
/// 解题思路：滑动窗口
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    // 定义慢指针
    let mut slow = 0;
    // 最大连续 1 的个数。
    let mut max_len = 0;
    // 最大子串 fast - slow + 1 = nums[slow] + nums[slow + 1]..nums[fast]
    for fast in 0..nums.len() {
        // 统计 nums[slow] + nums[slow + 1]..nums[fast] 的和
        let mut sum = 0;
        for i in slow..=fast {
            sum += nums[i];
        }

        while (slow < fast) && (fast - slow + 1) as i32 != sum {
            slow += 1;
        }
        max_len = max_len.max(sum);
    }
    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        let st = "pwwkew".to_string();
        let length = length_of_longest_substring(st);
        println!("{:?}", length)
    }

    #[test]
    fn test_min_sub_array_len() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];
        let length = min_sub_array_len(target, nums);
        println!("{:?}", length)
    }

    #[test]
    fn test_find_max_consecutive_ones() {
        let nums = [1, 0, 1, 1, 0, 1].to_vec();
        let find_max_consecutive_ones = find_max_consecutive_ones(nums);
        println!("{:?}", find_max_consecutive_ones)
    }
}