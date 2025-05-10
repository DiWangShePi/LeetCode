# 321. Create Maximum Number

### Description

You are given two integer arrays nums1 and nums2 of lengths m and n respectively. nums1 and nums2 represent the digits of two numbers. You are also given an integer k.

Create the maximum number of length k <= m + n from digits of the two numbers. The relative order of the digits from the same array must be preserved.

Return an array of the k digits representing the answer.

### Example 

###### Example I

```
Input: nums1 = [3,4,6,5], nums2 = [9,1,2,5,8,3], k = 5
Output: [9,8,6,5,3]
```

###### Example II

```
Input: nums1 = [6,7], nums2 = [6,0,4], k = 5
Output: [6,7,6,0,4]
```

###### Example II

```
Input: nums1 = [3,9], nums2 = [8,9], k = 3
Output: [9,8,9]
```

### Solution

枚举从两个数组中分别选取多少个元素（总数为 k），然后从每个数组中贪心地选出字典序最大的子序列，再通过比较字典序将两个子序列合并成一个新的候选结果，最终从所有候选结果中选出字典序最大的作为答案。这个过程结合了贪心选择最大子序列和字典序合并的策略，确保最终结果在所有可能中是字典序最大的。

```c++
class Solution {
public:
    vector<int> maxNumber(vector<int>& nums1, vector<int>& nums2, int k) {
        int m = nums1.size(), n = nums2.size();
        vector<int> maxSub(k, 0);
        int start = max(0, k - n), end = min(k, m);
        for (int i = start; i <= end; i++) {
            vector<int> sub1 = maxSubsequence(nums1, i);
            vector<int> sub2 = maxSubsequence(nums2, k - i);
            vector<int> cur = merge(sub1, sub2);
            if (compare(cur, 0, maxSub, 0) > 0) maxSub = cur;
        }
        return maxSub;
    }

    vector<int> maxSubsequence(vector<int>& nums, int k) {
        vector<int> stack;
        int remain = nums.size() - k;
        for (int num : nums) {
            while (!stack.empty() && stack.back() < num && remain > 0) {
                stack.pop_back();
                remain--;
            }
            stack.push_back(num);
        }
        return vector<int>(stack.begin(), stack.begin() + k);
    }

    vector<int> merge(vector<int>& sub1, vector<int>& sub2) {
        vector<int> merged;
        int i = 0, j = 0;
        while (i < sub1.size() || j < sub2.size()) {
            if (i < sub1.size() && (j >= sub2.size() || sub1[i] > sub2[j])) {
                merged.push_back(sub1[i++]);
            } else if (j < sub2.size() && (i >= sub1.size() || sub2[j] > sub1[i])) {
                merged.push_back(sub2[j++]);
            } else {
                if (lexicographical_compare(sub1.begin() + i, sub1.end(), sub2.begin() + j, sub2.end())) {
                    merged.push_back(sub2[j++]);
                } else {
                    merged.push_back(sub1[i++]);
                }
            }
        }
        return merged;
    }

    int compare(vector<int>& sub1, int index1, vector<int>& sub2, int index2) {
        while (index1 < sub1.size() && index2 < sub2.size()) {
            int diff = sub1[index1] - sub2[index2];
            if (diff != 0) return diff;
            index1++;
            index2++;
        }
        return (sub1.size() - index1) - (sub2.size() - index2);
    }
};
```
