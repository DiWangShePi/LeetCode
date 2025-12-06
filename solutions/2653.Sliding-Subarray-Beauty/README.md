# 2653. Sliding Subarray Beauty

**Tags:** Sliding Window, Heap, Multiset

### Description

Given an integer array nums containing n integers, find the beauty of each subarray of size k.

The beauty of a subarray is the xth smallest integer in the subarray if it is negative, or 0 if there are fewer than x negative integers.

Return an integer array containing n - k + 1 integers, which denote the beauty of the subarrays in order from the first index in the array.

A subarray is a contiguous non-empty sequence of elements within an array.

### Example

###### Example I

> Input: nums = [1,-1,-3,-2,3], k = 3, x = 2
> Output: [-1,-2,-2]
> Explanation: There are 3 subarrays with size k = 3. 
> The first subarray is [1, -1, -3] and the 2nd smallest negative integer is -1. 
> The second subarray is [-1, -3, -2] and the 2nd smallest negative integer is -2. 
> The third subarray is [-3, -2, 3] and the 2nd smallest negative integer is -2.

###### Example II

> Input: nums = [-1,-2,-3,-4,-5], k = 2, x = 2
> Output: [-1,-2,-3,-4]
> Explanation: There are 4 subarrays with size k = 2.
> For [-1, -2], the 2nd smallest negative integer is -1.
> For [-2, -3], the 2nd smallest negative integer is -2.
> For [-3, -4], the 2nd smallest negative integer is -3.
> For [-4, -5], the 2nd smallest negative integer is -4. 

###### Example III

> Input: nums = [-3,1,2,-3,0,-3], k = 2, x = 1
> Output: [-3,0,-3,-3,-3]
> Explanation: There are 5 subarrays with size k = 2.
> For [-3, 1], the 1st smallest negative integer is -3.
> For [1, 2], there is no negative integer so the beauty is 0.
> For [2, -3], the 1st smallest negative integer is -3.
> For [-3, 0], the 1st smallest negative integer is -3.
> For [0, -3], the 1st smallest negative integer is -3.

### Solution

这一题虽然和滑动数组有关，但其难点不在于滑动数组，而是如何在这个数组中快速的找到第 X 小的值。

每次更新数组后排序是一个十分麻烦的方式，他的开销会很大。

一个优雅的解法是用两个堆，维护大顶堆的长度为 K。但这对于更新有点麻烦，所以我们需要一个字典来维护需要删除的值。

```c++
class Solution {
public:
    vector<int> getSubarrayBeauty(vector<int>& nums, int k, int x) {
        priority_queue<int,vector<int>,greater<int>>small;
        priority_queue<int,vector<int>,less<int>>big;
        unordered_map<int,int>mp;
        int n=nums.size(),t;
        for(int i=0;i<k;i++)
        {
            big.push(nums[i]);
            if(big.size()>x)
            {
                small.push(big.top());
                big.pop();
            }
        }
        vector<int>res{big.top()<0?big.top():0};
        for(int i=k;i<n;i++)
        {
            bool p=nums[i-k]<=big.top();//删除的元素是否在左边
            mp[nums[i-k]]++;
            while(!big.empty()&&mp[t=big.top()])
            {
                mp[t]--;
                big.pop();
            }
            while(!small.empty()&&mp[t=small.top()])
            {
                mp[t]--;
                small.pop();
            }
            bool q=big.empty()||nums[i]>big.top();//是否在右边新加元素
            if(p&&q)
            {
                //左边删右边加
                small.push(nums[i]);
                big.push(small.top());
                small.pop();
                res.push_back((t=big.top())<0?t:0);
            }
            else if(p&&!q)
            {
                //左边删左边加
                big.push(nums[i]);
                res.push_back((t=big.top())<0?t:0);
            }
            else if(!p&&q)
            {
                //右边删，右边加
                small.push(nums[i]);
                res.push_back((t=big.top())<0?t:0);
            }
            else if(!p&&!q)
            {
                //右边删左边加
                big.push(nums[i]);
                small.push(big.top());
                big.pop();
                while(mp[big.top()])
                {
                    mp[big.top()]--;
                    big.pop();
                }
                res.push_back((t=big.top())<0?t:0);
            }
        }
        return res;
    }
};
```

但这写起来太麻烦了，c++给我们提供了 multiset，也可以实现这一功能。

```c++
#include <ext/pb_ds/assoc_container.hpp>

using namespace __gnu_pbds;

// 使用 pair<value, index> 支持重复元素
using ordered_set = tree<pair<int, int>, null_type, less<>, rb_tree_tag, tree_order_statistics_node_update>;

class Solution {
public:
    vector<int> getSubarrayBeauty(vector<int>& nums, int k, int x) {
        ordered_set st;
        for (int i = 0; i < k - 1; i++) {
            st.insert({nums[i], i});
        }

        int n = nums.size();
        vector<int> ans(n - k + 1);
        for (int i = k - 1; i < n; i++) {
            st.insert({nums[i], i});
            auto [v, _] = *st.find_by_order(x - 1); // 第 x 小
            ans[i - k + 1] = min(v, 0);
            st.erase({nums[i - k + 1], i - k + 1});
        }
        return ans;
    }
};
```

考虑到这一题给的元素范围，我们也可以维护一个元素出现次数的表格，每次遍历表格找到第X小的值。

```c++
class Solution {
public:
    vector<int> getSubarrayBeauty(vector<int>& nums, int k, int x) {
        vector<int> dict(101, 0);
        vector<int> an;

        int i = 0;
        for ( ; i < k; i++) dict[nums[i] + 50]++;
        an.push_back(find(dict, x));
        for ( ; i < nums.size(); i++) {
            dict[nums[i] + 50]++;
            dict[nums[i - k] + 50]--;

            an.push_back(find(dict, x));
        }
        return an;
    }

private:
    int find(vector<int>& dict, int x) {
        int sum = 0;
        for (int i = 0; i < dict.size(); i++) {
            sum += dict[i];
            if (sum >= x) {
                if (i < 50) return i - 50;
                else return 0;
            }
        }
        return 0;
    }
};
```
