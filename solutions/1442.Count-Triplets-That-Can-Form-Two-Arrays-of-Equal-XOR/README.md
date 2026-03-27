# 1442. Count Triplets That Can Form Two Arrays of Equal XOR

**Tags:** Bitwise Operation, Prefix Sum

### Description

Given an array of integers arr.

We want to select three indices i, j and k where (0 <= i < j <= k < arr.length).

Let's define a and b as follows:

- a = arr[i] ^ arr[i + 1] ^ ... ^ arr[j - 1]
- b = arr[j] ^ arr[j + 1] ^ ... ^ arr[k]
Note that ^ denotes the bitwise-xor operation.

Return the number of triplets (i, j and k) Where a == b.

### Example

###### Example I

> Input: arr = [2,3,1,6,7]
> Output: 4
> Explanation: The triplets are (0,1,2), (0,2,2), (2,3,4) and (2,4,4)

###### Example II

> Input: arr = [1,1,1,1,1]
> Output: 10

### Solution

注意到数组的大小其实不大，所以我们可以用前缀异或和做遍历。
注意到当 a == b 时，其实 prefix[j - 1] 在等式两都存在，所以我们只需要一个二重循环就可以了。

```c++
class Solution {
public:
    int countTriplets(vector<int>& arr) {
        vector<int> prefix{0};
        for (int& a : arr) prefix.push_back(prefix.back() ^ a);
        
        int n = arr.size(), ans = 0;
        for (int i = 0; i < n; i++) {
            for (int k = i + 1; k < n; k++) {
                if (prefix[i] == prefix[k + 1]) {
                    ans += (k - i);  
                }
            }
        }
        return ans;
    }
};
```
