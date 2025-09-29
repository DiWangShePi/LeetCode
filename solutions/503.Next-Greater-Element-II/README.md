# 503. Next Greater Element II

**Tags:** Stack

### Description

Given a circular integer array nums (i.e., the next element of nums[nums.length - 1] is nums[0]), return the next greater number for every element in nums.

The next greater number of a number x is the first greater number to its traversing-order next in the array, which means you could search circularly to find its next greater number. If it doesn't exist, return -1 for this number.

### Example 

###### Example I

> Input: nums = [1,2,1]
> Output: [2,-1,2]
> Explanation: The first 1's next greater number is 2; 
> The number 2 can't find next greater number. 
> The second 1's next greater number needs to search circularly, which is also 2.

###### Example II

> Input: nums = [1,2,3,4,3]
> Output: [2,3,4,-1,4]

### Solution

先给一个暴力的，O(n^2)的两次遍历的解法

```c++
class Solution {
public:
    vector<int> nextGreaterElements(vector<int>& nums) {
        vector<int> an;
        for (int i = 0; i < nums.size(); i++) {
            an.push_back(search(nums, i));
        }
        return an;
    }

private:
    int search(vector<int>& nums, int index) {
        for (int i = 0; i < nums.size(); i++) {
            int j = (i + index) % nums.size();
            if (nums[j] > nums[index]) return nums[j];
        }
        return -1;
    }
};
```

或者我们也可以用栈来处理这个问题。遍历数组，如果当前元素大于栈顶的元素，那么该元素的 next greater element 就是当前元素。

题目要求是循环的，因此我们需要遍历数组两遍，以确保最后的元素也能找到合适的值。

```c++
class Solution {
public:
    vector<int> nextGreaterElements(vector<int>& nums) {
        vector<int> an(nums.size(), -1);
        stack<int> s;

        int index;
        for (int i = 0; i < 2 * nums.size(); i++) {
            index = i % nums.size();
            while (!s.empty() && nums[s.top()] < nums[index]) {
                an[s.top()] = nums[index];
                s.pop();
            }

            s.push(index);
        }
        return an;
    }
};
```
