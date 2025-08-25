# 456. 132 Pattern

### Description

Given an array of n integers nums, a 132 pattern is a subsequence of three integers nums[i], nums[j] and nums[k] such that i < j < k and nums[i] < nums[k] < nums[j].

Return true if there is a 132 pattern in nums, otherwise, return false.

### Example

###### Example I

> Input: nums = [1,2,3,4]
> Output: false
> Explanation: There is no 132 pattern in the sequence.

###### Example II

> Input: nums = [3,1,4,2]
> Output: true
> Explanation: There is a 132 pattern in the sequence: [1, 4, 2].

###### Example III

> Input: nums = [-1,3,2,0]
> Output: true
> Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1, 3, 0] and [-1, 2, 0].

### Solution

先给出一个暴力的解法：遍历一遍数组，获得当前元素向左构成的子数组中的最小值。再枚举所有 i，j 构成的 nums[i]和 nums[j]，存在满足 nums[i] > nums[j] 且 smallest[i] < nums[j]的情况即为满足条件。这是一个 O(n^2)的解法。

```c++
class Solution {
public:
    bool find132pattern(vector<int>& nums) {
        int n = nums.size();
        if (n < 3) return false;

        vector<int> smallest{nums[0]};
        for (int i = 1; i < n; i++)
            smallest.push_back(min(smallest[i - 1], nums[i]));

        for (int i = 1; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                if (nums[i] > nums[j] && smallest[i] < nums[j])
                    return true;
            }
        }
        return false;
    }
};
```

我们发现枚举 i,j,k 中的两个下标是一个必然会超时的做法，因此我们考虑枚举其中一个下标，再尝试维护另外两个。

显然 i 的维护最简单，我们在上面给出的解法中已经做到了。我们需要考虑如何维护 j 或者 k。

我们来尝试维护 k，用一个单调栈从左到右遍历，存放可能的 k 的候选值。如果当前值大于栈顶的元素，我们就可以不断的将元素出栈，并更新 k 可能的值（我们希望 k 尽可能的接近 j，这样为 i 留出空间）。

如果我们发现当前值小于 k，就代表我们发现了合法的序列，因为对于此时用于比较的 k，有比它大的 j 存在。

```c++
class Solution {
public:
    bool find132pattern(vector<int>& nums) {
        int n = nums.size();
        if (n < 3) return false;

        int second = INT_MIN;           
        stack<int> st;                  

        for (int i = n - 1; i >= 0; --i) {
            if (nums[i] < second) 
                return true;
                
            while (!st.empty() && nums[i] > st.top()) {
                second = st.top();
                st.pop();
            }
            st.push(nums[i]);
        }
        return false;
    }
};
```
