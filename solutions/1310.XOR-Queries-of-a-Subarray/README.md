# 1310. XOR Queries of a Subarray

**Tags:** Bitwise Operation

### Description

You are given an array arr of positive integers. You are also given the array queries where queries[i] = [lefti, righti].

For each query i compute the XOR of elements from lefti to righti (that is, arr[lefti] XOR arr[lefti + 1] XOR ... XOR arr[righti] ).

Return an array answer where answer[i] is the answer to the ith query.

### Example

###### Example I

> Input: arr = [1,3,4,8], queries = [[0,1],[1,2],[0,3],[3,3]]
> Output: [2,7,14,8] 
> Explanation: 
> The binary representation of the elements in the array are:
> 1 = 0001 
> 3 = 0011 
> 4 = 0100 
> 8 = 1000 
> The XOR values for queries are:
> [0,1] = 1 xor 3 = 2 
> [1,2] = 3 xor 4 = 7 
> [0,3] = 1 xor 3 xor 4 xor 8 = 14 
> [3,3] = 8

###### Example II

> Input: arr = [4,8,2,10], queries = [[2,3],[1,3],[0,0],[0,3]]
> Output: [8,0,4,4]

### Solution

异或的性质：一个数字和自己异或，得到的结果为0。一个数字和 0 异或，得到的结果为数字自己。

因此可以计算这个数字的异或前缀和。对于指定的查询[l, r]，记异或前缀和数组为 xor，查询的答案为 xor[l - 1] ^ xor[r]。

```c++
class Solution {
public:
    vector<int> xorQueries(vector<int>& arr, vector<vector<int>>& queries) {
        vector<int> prefix{0, arr[0]};
        int start = arr[0], n = arr.size();
        for (int i = 1; i < n; i++) {
            start ^= arr[i];
            prefix.push_back(start);
        }

        vector<int> ans;
        for (vector<int>& q : queries) {
            ans.push_back(prefix[q[0]] ^ prefix[q[1] + 1]);
        }
        return ans;
    }
};
```
