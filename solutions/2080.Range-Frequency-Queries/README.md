# 2080. Range Frequency Queries

**Tags:** Hash Table, Binary Search

### Description

Design a data structure to find the frequency of a given value in a given subarray.

The frequency of a value in a subarray is the number of occurrences of that value in the subarray.

Implement the RangeFreqQuery class:

- RangeFreqQuery(int[] arr) Constructs an instance of the class with the given 0-indexed integer array arr.
- int query(int left, int right, int value) Returns the frequency of value in the subarray arr[left...right].

A subarray is a contiguous sequence of elements within an array. arr[left...right] denotes the subarray that contains the elements of nums between indices left and right (inclusive).

### Example

###### Example I

> Input
> ["RangeFreqQuery", "query", "query"]
> [[[12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]], [1, 2, 4], [0, 11, 33]]
> Output
> [null, 1, 2]
> 
> Explanation
> RangeFreqQuery rangeFreqQuery = new RangeFreqQuery([12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
> rangeFreqQuery.query(1, 2, 4); // return 1. The value 4 occurs 1 time in the subarray [33, 4]
> rangeFreqQuery.query(0, 11, 33); // return 2. The value 33 occurs 2 times in the whole array.

### Solution

遍历数组，将每个数字和其相应的下标保存在哈希表中。当给定目标和区间时，在保存的下标数组中检查 left 和 right 相应的位置，从而计算有多少满足条件。

```c++
class RangeFreqQuery {
    unordered_map<int, vector<int>> dict;
public:
    RangeFreqQuery(vector<int>& arr) {
        for (int i = 0; i < arr.size(); i++)
            dict[arr[i]].push_back(i);
    }
    
    int query(int left, int right, int value) {
        const vector<int>& arr = dict[value];
        auto l = lower_bound(arr.begin(), arr.end(), left);
        auto r = upper_bound(arr.begin(), arr.end(), right);
        return r - l;
    }
};

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * RangeFreqQuery* obj = new RangeFreqQuery(arr);
 * int param_1 = obj->query(left,right,value);
 */
```

注意 lower bound 会返回第一个大于等于目标值的下标，upper bound 则会返回第一个大于目标值的下标。
