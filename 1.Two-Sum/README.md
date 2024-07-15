# 1. Two Sum

### 题目描述

给定一个整数数组 `nums` 和一个目标值 `target`，在该数组中找出和为目标值的 **两个** 整数，并返回他们的数组下标

你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素

**示例:**

```
给定 nums = [2, 7, 11, 15], target = 9

nums[0] + nums[1] = 2 + 7 = 9
所以返回 [0, 1]
```

### 题目解析

设置一个哈希表`record`用来记录元素的值与索引，遍历数组 `nums`

- 遍历时，对每个获取到的整数，计算临时变量`complement`为目标值与整数的差值
- 在哈希表`record`中查找`complement`是否存在：
  - 如果存在，返还当前整数下标与哈希表中索引为`complement`的元素
  - 如果不存在，在哈希表中记录当前整数与当前整数的下标


### 代码实现

###### C++

``` c++
#include <iostream>
#include <map>

using namespace std;

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        std::map<int, int> record;
        int complementary;

        for (int i=0; i < nums.size(); i++) {
            complementary = target - nums[i];

            if (record.count(complementary) != 0) {
                return {i, record[complementary]};
            } else {
                record[nums[i]] = i;
            } 
        }

        return {};
    }
};
```

###### Rust

``` rust
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut record = HashMap::new(); 
        let mut complementary;

        for i in 0..nums.len() {
            complementary = target - nums[i];

            if let Some(&index) = record.get(&complementary) { 
                return vec![index as i32, i as i32]; 
            } else {
                record.insert(nums[i], i as i32);
            }
        }

        vec![] 
    }
}
```