# 2389. Longest Subsequence With Limited Sum

**Tags:** Binary Search

### Description

You are given an integer array nums of length n, and an integer array queries of length m.

Return an array answer of length m where answer[i] is the maximum size of a subsequence that you can take from nums such that the sum of its elements is less than or equal to queries[i].

A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.

### Example

###### Example I

> Input: nums = [4,5,2,1], queries = [3,10,21]
> Output: [2,3,4]
> Explanation: We answer the queries as follows:
> - The subsequence [2,1] has a sum less than or equal to 3. It can be proven that 2 is the maximum size of such a > subsequence, so answer[0] = 2.
> - The subsequence [4,5,1] has a sum less than or equal to 10. It can be proven that 3 is the maximum size of such a > subsequence, so answer[1] = 3.
> - The subsequence [4,5,2,1] has a sum less than or equal to 21. It can be proven that 4 is the maximum size of such a > subsequence, so answer[2] = 4.

###### Example II

> Input: nums = [2,3,4,5], queries = [1]
> Output: [0]
> Explanation: The empty subsequence is the only subsequence that has a sum less than or equal to 1, so answer[0] = 0.

### Solution

将给定的数组 nums 排序一遍，求前缀和，然后枚举 queries 中的数字在前缀和中进行查询。

```c++
class Solution {
public:
    vector<int> answerQueries(vector<int>& nums, vector<int>& queries) {
        sort(nums.begin(), nums.end());
        vector<int> prefix{nums[0]};
        for (int i = 1; i < nums.size(); i++) {
            prefix.push_back(prefix.back() + nums[i]);
        }

        vector<int> answer;
        for (int q : queries) {
            int index = find(prefix, q);
            answer.push_back(index);
        }
        return answer;
    }

private:
    int find(vector<int>& pre, int t) {
        int l = 0, r = pre.size() - 1;
        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (pre[mid] == t) return mid + 1;
            else if (pre[mid] > t) r = mid - 1;
            else l = mid + 1;
        }
        return l;
    }
};
```
