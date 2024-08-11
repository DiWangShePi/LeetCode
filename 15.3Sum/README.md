# 15. 3Sum

### 题目描述

给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请你返回所有和为 0 且不重复的三元组。

注意：答案中不可以包含重复的三元组。

**示例：**

```
输入：nums = [-1,0,1,2,-1,-4]
输出：[[-1,-1,2],[-1,0,1]]
解释：
nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
注意，输出的顺序和三元组的顺序并不重要。
```

```
输入：nums = [0,1,1]
输出：[]
解释：唯一可能的三元组和不为 0 。
```

```
输入：nums = [0,0,0]
输出：[[0,0,0]]
解释：唯一可能的三元组和为 0 。
```

### 题目解析

延用第一题的思路。遍历数组，对每个元素I，遍历除I以外所有元素J，在字典中记录0与I,J之和的差值。

遍历时同步检查当前元素是否存在于数组。

### 代码实现

###### c++
```c++
#include <vector>
#include <algorithm>

class Solution {
public:
    std::vector<std::vector<int>> threeSum(std::vector<int>& nums) {
        std::vector<std::vector<int>> answer;
        std::sort(nums.begin(), nums.end());

        for (int i = 0; i < nums.size(); i++) {
            if (i > 0 && nums[i] == nums[i - 1]) continue;

            int left = i + 1, right = nums.size() - 1;

            while (left < right) {
                int sum = nums[i] + nums[left] + nums[right];

                if (sum == 0) {
                    answer.push_back({nums[i], nums[left], nums[right]});
                    while (left < right && nums[left] == nums[left + 1]) left++;
                    while (left < right && nums[right] == nums[right - 1]) right--;
                    left++;
                    right--;
                } else if (sum < 0) {
                    left++;
                } else {
                    right--;
                }
            }
        }

        return answer;
    }
};
```

###### rust

```rust
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
         let mut nums = nums.clone();
        nums.sort();
        let mut answer = Vec::new();
        let n = nums.len();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut left, mut right) = (i + 1, n - 1);

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum == 0 {
                    answer.push(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        answer
    }
}
```
