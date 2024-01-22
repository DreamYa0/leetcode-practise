// 冒泡排序
pub fn bubble_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let len = nums.len();
    // 记录最后一次交换的位置
    let mut last_exchange_index = 0;
    // 无序数列的边界，每次比较只需要比到这里为止
    let mut sort_border = len - 1;
    for _ in 0..len - 1 {
        // 有序标记，每一轮的初始是true
        let mut is_sorted = true;
        for j in 0..sort_border {
            if nums[j] > nums[j + 1] {
                // 交换
                let temp = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = temp;
                // 有元素交换，所以不是有序，标记变为false
                is_sorted = false;
                // 把无序数列的边界更新为最后一次交换元素的位置
                last_exchange_index = j;
            }
        }
        // 更新无序数列的边界为最后一次交换元素的位置
        sort_border = last_exchange_index;
        // 如果有序标记为true，说明本轮遍历没有交换，已经是有序数列，直接跳出大循环
        if is_sorted {
            break;
        }
    }
    nums
}

// 快速排序
pub fn quick_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let len = nums.len();
    do_quick_sort(&mut nums, 0, (len - 1) as i32);
    nums
}

fn do_quick_sort(nums: &mut Vec<i32>, start: i32, end: i32) {
    // 递归结束条件
    if start >= end {
        return;
    }
    // 得到基准元素位置
    let pivot_index = partition(nums, start, end);
    // 根据基准元素，分成两部分递归排序
    do_quick_sort(nums, start, pivot_index - 1);
    do_quick_sort(nums, pivot_index+1, end);
}

fn partition(arr: &mut Vec<i32>, start_index: i32, end_index: i32) -> i32 {
    // 取第一个位置的元素作为基准元素
    let pivot = arr[start_index as usize];
    let (mut left, mut right) = (start_index as usize, end_index as usize);
    while left != right {
        // 控制right指针比较并左移,右边的数大于基准数则指针左移，如果小于或等于基准数则停止移动，等待被交换
        while left < right && arr[right] > pivot {
            right -= 1;
        }
        // 控制right指针比较并右移，左边的数小于或等于基准数则指针右移，如果大于基准数则停止移动，等待被交换
        while left < right && arr[left] <= pivot {
            left += 1;
        }
        // 交换left和right指向的元素
        if left < right {
            let temp = arr[left];
            arr[left] = arr[right];
            arr[right] = temp;
        }
    }
    // pivot和指针重合点交换
    arr[start_index as usize] = arr[left];
    arr[left] = pivot;
    left as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array() {
        let nums = vec![5, 2, 3, 1];
        let res = bubble_sort(nums);
        assert_eq!(res, vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_quick_sort() {
        let nums = vec![5, 2, 3, 1];
        let res = quick_sort(nums);
        assert_eq!(res, vec![1, 2, 3, 5]);
    }
}
