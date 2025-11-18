# 1461. Check If a String Contains All Binary Codes of Size K

### Description

Given a binary string s and an integer k, return true if every binary code of length k is a substring of s. Otherwise, return false.

### Example

###### Example I

> Input: s = "00110110", k = 2
> Output: true
> Explanation: The binary codes of length 2 are "00", "01", "10" and "11". They can be all found as substrings at indices 0, 1, 3 and 2 respectively.

###### Example II

> Input: s = "0110", k = 1
> Output: true
> Explanation: The binary codes of length 1 are "0" and "1", it is clear that both exist as a substring. 

###### Example III

> Input: s = "0110", k = 2
> Output: false
> Explanation: The binary code "00" is of length 2 and does not exist in the array.

### Solution

长度为 K 的滑动窗口滑一遍，哈希表记录结果，看最后哈希表的大小是否等于 2^K

```c++
class Solution {
public:
    bool hasAllCodes(string s, int k) {
        if (s.size() < (1 << k) + k - 1) {
            return false;
        }

        int num = stoi(s.substr(0, k), nullptr, 2);
        unordered_set<int> exists = {num};
        
        for (int i = 1; i + k <= s.size(); ++i) {
            num = (num - ((s[i - 1] - '0') << (k - 1))) * 2 + (s[i + k - 1] - '0');
            exists.insert(num);
        }
        return exists.size() == (1 << k);
    }
};
```
