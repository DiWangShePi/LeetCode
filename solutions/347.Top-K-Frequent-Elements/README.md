# 347. Top K Frequent Elements

### Description

Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.

### Example 

###### Example I

```
Input: nums = [1,1,1,2,2,3], k = 2
Output: [1,2]
```

###### Example II

```
Input: nums = [1], k = 1
Output: [1]
```

### Solution

遍历一遍，用字典计数，在字典中找最大的数字，重复K次。

```c++
class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        unordered_map<int, int> dict;
        for (int num : nums) dict[num]++;

        vector<int> an;
        for (int i = 0; i < k; i++) an.push_back( findMax(dict));
        return an;
    }

private:
    int findMax(unordered_map<int, int>& dict) {
        auto maxIt = std::max_element(
            dict.begin(),
            dict.end(),
            [](const auto& a, const auto& b) {
                return a.second < b.second; 
            }
        );
        maxIt->second = 0;
        return maxIt->first;
    }
};
```

也可以将字典中的值放入pair中，对pair排序，再取前K个。

```c++
class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        unordered_map<int, int> dict;
        vector<int> an;
        vector<pair<int, int>> pairs;

        for (int num : nums) dict[num]++;

        for (const pair<int, int>& ele : dict) {
            pairs.push_back(make_pair(ele.second, ele.first));
        }
        sort(pairs.begin(), pairs.end());

        for (int i = 1; i <= k; i++) an.push_back(pairs[pairs.size() - i].second);
        return an;
    }
};
```
