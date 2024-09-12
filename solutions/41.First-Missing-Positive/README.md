# 41. First Missing Positive

### 题目描述

给定一个未排序的数组nums，返回nums中不存在的最小正整数。

你的解法必须是O(N)的时间复杂度和O(1)的空间复杂度。

**示例：**

```
Input: nums = [1,2,0]
Output: 3
Explanation: The numbers in the range [1,2] are all in the array.
```

```
Input: nums = [3,4,-1,1]
Output: 2
Explanation: 1 is in the array but 2 is missing.
```

```
Input: nums = [7,8,9,11,12]
Output: 1
Explanation: The smallest positive integer 1 is missing.
```

### 题目解析

一个非常直观的解决方法，就是先将数组排序一遍，然后遍历数组。
我们记录一个值，起初始值为1,如果遇到了与当前值一样的数字，则将该值加1（忽略所有小于等于0的数字）。
若某一次比较时该值与数组中的元素不等，此时的值即为缺失的元素。

但这并不满足我们的时间复杂度需求。于是我们尝试改进这个算法。我们可以初始化一个与数组大小一致的哈希表。
同样忽略小于等于零的数字，遍历一边数组，当数字存在于哈希表中时，将该为在哈希表中指示为true。
遍历结束后再遍历哈希表，当哈希表中的某一个元素为false时，该元素即为缺失的。若全部存在，则返回哈希表的大小+1。

这样我们满足了时间复杂度的需求，但不满足空间复杂度的需求。
我们进一步的思考：其实数组中元素的符号也可以用于指示一个元素存在与否，只要我们建立一个简单的映射关系。
如，若数字3存在，则将3-1下标的数字变为负数（考虑到给定的数组中存在负数，则我们需要先剔除掉所有的负数）。
这样，当我们遍历完成一遍数组后，若某一个元素依旧为正值，即代表我们并未找到该下标加1的数字。

### 代码实现

###### c++

```c++
class Solution {
public:
    int firstMissingPositive(vector<int>& nums) {

        nums.erase(remove_if(nums.begin(), nums.end(), [](int n) { return n <= 0; }), nums.end());

        for (int i = 0; i < nums.size(); i++) {
            int n = abs(nums[i]);
            if (n <= nums.size() && nums[n - 1] > 0) {
                nums[n - 1] = -nums[n - 1];
            }
        }

        for (int i = 0; i < nums.size(); i++) {
            if (nums[i] > 0) {
                return i + 1;
            }
        }

        return nums.size() + 1;        
    }
};
```