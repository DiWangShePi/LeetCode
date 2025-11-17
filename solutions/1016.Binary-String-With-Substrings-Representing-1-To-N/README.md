# 1016. Binary String With Substrings Representing 1 To N

### Description

Given a binary string s and a positive integer n, return true if the binary representation of all the integers in the range [1, n] are substrings of s, or false otherwise.

A substring is a contiguous sequence of characters within a string.

### Example

###### Example II

> Input: s = "0110", n = 3
> Output: true

###### Example III

> Input: s = "0110", n = 4
> Output: false

### Solution

我一开始想到的办法是枚举 S 所能构成的子字符串，将他们转换为数字，然后遍历查找是否都存在。

```c++
class Solution {
public:
    bool queryString(string s, int n) {
        unordered_map<long long, int> dict;
        int len = s.size();
        for (int size = 1; size <= min(32, len); size++) {
            for (int i = 0; i + size <= s.size(); i++) {
                dict[stoll(s.substr(i, size), nullptr, 2)] = 1;
            }
        }

        for (int i = 1; i <= n; i++) {
            if (dict.count(i) == 0) return false;
        }
        return true;
    }
};
```

随后发现，如果我们从1到n枚举每个数字，计算其二进制字符串，再在 s 中查找，也能比这个快很多。

```c++
class Solution {
public:
    bool queryString(string s, int n) {
        for (int i = 1; i <= n; ++i) {
            string bin;
            for (int t = i; t; t >>= 1) 
                bin.push_back('0' + (t & 1));
            
            reverse(bin.begin(), bin.end());
            if (s.find(bin) == string::npos) 
                return false;
        }
        return true;
    }
};
```

对于我一开始的想法，一个很大的开销在每一次转数字上，针对这一点做优化，我们就可以更快的获得所有可能的数字。

```c++
class Solution {
public:
    bool queryString(string s, int n) {
        unordered_set<int> seen;
        for (int i = 0, m = s.length(); i < m; ++i) {
            int x = s[i] - '0';
            if (x == 0) continue; // 二进制数从 1 开始
            for (int j = i + 1; x <= n; j++) {
                seen.insert(x);
                if (j == m) break;
                x = (x << 1) | (s[j] - '0'); // 子串 [i,j] 的二进制数
            }
        }
        return seen.size() == n;
    }
};
```

由于这里我们加入了限制条件（只检查范围在1到n之间的数字），且子字符串的增长幅度是指数级别的。因此内循环的复杂度是 logN 而不是 N.

第三种解法挺有意思的，参考：https://leetcode.cn/problems/binary-string-with-substrings-representing-1-to-n/solutions/2265097/san-chong-suan-fa-cong-bao-li-dao-you-hu-nmtq/

```c++
class Solution {
public:
    bool queryString(string s, int n) {
        if (n == 1)
            return s.find('1') != string::npos;

        int m = s.length();
        int k = 31 - __builtin_clz(n); // n 的二进制长度减一
        if (m < max(n - (1 << k) + k + 1, (1 << (k - 1)) + k - 1))
            return false;

        // 对于长为 k 的在 [lower, upper] 内的二进制数，判断这些数 s 是否都有
        auto check = [&](int k, int lower, int upper) -> bool {
            if (lower > upper) return true;
            unordered_set<int> seen;
            int mask = (1 << (k - 1)) - 1;
            int x = stoi(s.substr(0, k - 1), nullptr, 2);
            for (int i = k - 1; i < m; i++) {
                // & mask 可以去掉最高比特位，从而实现滑窗的「出」
                // << 1 | (s[i] - '0') 即为滑窗的「入」
                x = ((x & mask) << 1) | (s[i] - '0');
                if (lower <= x && x <= upper)
                    seen.insert(x);
            }
            return seen.size() == upper - lower + 1;
        };

        return check(k, n / 2 + 1, (1 << k) - 1) && check(k + 1, 1 << k, n);
    }
};
```
