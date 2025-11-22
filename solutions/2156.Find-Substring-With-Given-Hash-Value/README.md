# 2156. Find Substring With Given Hash Value

### Description

The hash of a 0-indexed string s of length k, given integers p and m, is computed using the following function:

- hash(s, p, m) = (val(s[0]) * p0 + val(s[1]) * p1 + ... + val(s[k-1]) * pk-1) mod m.
Where val(s[i]) represents the index of s[i] in the alphabet from val('a') = 1 to val('z') = 26.

You are given a string s and the integers power, modulo, k, and hashValue. Return sub, the first substring of s of length k such that hash(sub, power, modulo) == hashValue.

The test cases will be generated such that an answer always exists.

A substring is a contiguous non-empty sequence of characters within a string.

### Example

###### Example I

> Input: s = "leetcode", power = 7, modulo = 20, k = 2, hashValue = 0
> Output: "ee"
> Explanation: The hash of "ee" can be computed to be hash("ee", 7, 20) = (5 * 1 + 5 * 7) mod 20 = 40 mod 20 = 0. 
> "ee" is the first substring of length 2 with hashValue 0. Hence, we return "ee".

###### Example II

> Input: s = "fbxzaad", power = 31, modulo = 100, k = 3, hashValue = 32
> Output: "fbx"
> Explanation: The hash of "fbx" can be computed to be hash("fbx", 31, 100) = (6 * 1 + 2 * 31 + 24 * 312) mod 100 = 23132 mod 100 = 32. 
> The hash of "bxz" can be computed to be hash("bxz", 31, 100) = (2 * 1 + 24 * 31 + 26 * 312) mod 100 = 25732 mod 100 = 32. 
> "fbx" is the first substring of length 3 with hashValue 32. Hence, we return "fbx".
> Note that "bxz" also has a hash of 32 but it appears later than "fbx".

### Solution

滑动窗口，从右往左滑动，每次剪掉最右边的字符的计算值，再加上左边字符的计算值。

```c++
class Solution {
public:
    string subStrHash(string s, int power, int modulo, int k, int hashValue) {
        int n = s.size();
        long long hash = 0;
        long long maxPower = 1;

        for (int i = 0; i < k - 1; i++) {
            maxPower = (maxPower * power) % modulo;
        }

        for (int i = n - 1; i >= n - k; i--) {
            hash = (hash * power + val(s[i])) % modulo;
        }
        
        string result = "";
        if (hash == hashValue) result = s.substr(n - k, k);
        for (int i = n - k - 1; i >= 0; i--) {
            hash = (hash - val(s[i + k]) * maxPower % modulo + modulo) % modulo;
            hash = (hash * power + val(s[i])) % modulo;
            
            if (hash == hashValue) {
                result = s.substr(i, k);
            }
        }
        
        return result;
    }

private:
    long long val(char c) {
        return c - 'a' + 1;
    }
};
```
