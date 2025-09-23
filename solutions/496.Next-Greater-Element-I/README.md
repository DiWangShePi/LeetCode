# 496. Next Greater Element I

**Tags:** Stack

### Description

The next greater element of some element x in an array is the first greater element that is to the right of x in the same array.

You are given two distinct 0-indexed integer arrays nums1 and nums2, where nums1 is a subset of nums2.

For each 0 <= i < nums1.length, find the index j such that nums1[i] == nums2[j] and determine the next greater element of nums2[j] in nums2. If there is no next greater element, then the answer for this query is -1.

Return an array ans of length nums1.length such that ans[i] is the next greater element as described above.

### Example

###### Example I

> Input: nums1 = [4,1,2], nums2 = [1,3,4,2]
> Output: [-1,3,-1]
> Explanation: The next greater element for each value of nums1 is as follows:
> - 4 is underlined in nums2 = [1,3,4,2]. There is no next greater element, so the answer is -1.
> - 1 is underlined in nums2 = [1,3,4,2]. The next greater element is 3.
> - 2 is underlined in nums2 = [1,3,4,2]. There is no next greater element, so the answer is -1.

###### Example II

> Input: nums1 = [2,4], nums2 = [1,2,3,4]
> Output: [3,-1]
> Explanation: The next greater element for each value of nums1 is as follows:
> - 2 is underlined in nums2 = [1,2,3,4]. The next greater element is 3.
> - 4 is underlined in nums2 = [1,2,3,4]. There is no next greater element, so the answer is -1.

### Solution

两次遍历得到 nums2 中，当前数字右边第一个比它大的数字，然后 nums1 遍历查询，没查到就是 -1.

```c++
class Solution {
public:
    vector<int> nextGreaterElement(vector<int>& nums1, vector<int>& nums2) {
        int n = nums2.size();
        unordered_map<int, int> dict;
        for (int i = 0; i < n; i++) {
            for (int j = i; j < n; j++) {
                if (nums2[j] > nums2[i]) {
                    dict[nums2[i]] = nums2[j];
                    break;
                }
            }
        }
        
        int m = nums1.size();
        vector<int> an(m);
        for (int i = 0; i < m; i++) {
            if (dict.count(nums1[i]) != 0) an[i] = dict[nums1[i]];
            else an[i] = -1;
        }
        return an;
    }
};
```

一个更有意思的做法是用栈来处理这个问题，如果栈顶的元素小于当前元素，那么栈顶元素的目标值就是当前元素。

```c++
class Solution {
public:
    vector<int> nextGreaterElement(vector<int>& nums1, vector<int>& nums2) {
        stack<int> s;
        unordered_map<int, int> dict;
        for (int num : nums2) {
            while (!s.empty() && s.top() < num) {
                dict[s.top()] = num;
                s.pop();
            }
            s.push(num);
        }
        while (!s.empty()) {
            dict[s.top()] = -1;
            s.pop();
        }

        vector<int> an;
        for (int num : nums1) {
           an.push_back(dict[num]);
        }
        return an;
    }
};
```

我一开始给出的版本（下面这个），没有上面这份快，不知道原因，我感觉是差不多的

```c++
class Solution {
public:
    vector<int> nextGreaterElement(vector<int>& nums1, vector<int>& nums2) {
        stack<int> s;
        unordered_map<int, int> dict;
        for (int num : nums2) {
            while (!s.empty() && s.top() < num) {
                dict[s.top()] = num;
                s.pop();
            }
            s.push(num);
        }

        vector<int> an;
        for (int num : nums1) {
            if (dict.count(num) != 0) an.push_back(dict[num]);
            else an.push_back(-1);
        }
        return an;
    }
};
```
