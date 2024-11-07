# 93. Restore IP Addresses

### Questionn Description

A valid IP address consists of exactly four integers separated by single dots. Each integer is between 0 and 255 (inclusive) and cannot have leading zeros.

For example, "0.1.2.201" and "192.168.1.1" are valid IP addresses, but "0.011.255.245", "192.168.1.312" and "192.168@1.1" are invalid IP addresses.
Given a string s containing only digits, return all possible valid IP addresses that can be formed by inserting dots into s. You are not allowed to reorder or remove any digits in s. You may return the valid IP addresses in any order.

**Example: **

```
Input: s = "25525511135"
Output: ["255.255.11.135","255.255.111.35"]
```

```
Input: s = "0000"
Output: ["0.0.0.0"]
```

```
Input: s = "101023"
Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
```

### Solution

与上一题一致，限制变为一次走一步，两步或三步，总共走四步。
- 当前字母为0时，仅能走一步。
- 当前字母为1时，可以走一步，两步或三步。
- 当前字母为2时，可以走一步或两步，是否能走第三步需要看后两位数字均在0到5之间
- 当前字母大于等于3时，能走一步或两步。

### Code Implemption

###### c++

```c++
class Solution {
public:
    vector<string> restoreIpAddresses(string s) {
        vector<string> answer;
        if (s.length() < 4 || s.length() > 12) return answer;  
        dfs(1, 1, s, s.substr(0, 1), answer);

        if (s[0] != '0') {
            dfs(1, 2, s, s.substr(0, 2), answer);
            if (isValidSegment(s.substr(0, 3))) { 
                dfs(1, 3, s, s.substr(0, 3), answer);
            }
        }

        return answer;
    }

    void dfs(int depth, int index, string& s, string current, vector<string>& answer) {
        if (depth == 4 && index == s.length()) {
            answer.push_back(current);
            return;
        }
        if (index >= s.length() || depth >= 4) return;

        dfs(depth + 1, index + 1, s, current + '.' + s.substr(index, 1), answer);

        if (s[index] != '0' && index + 1 < s.length()) {
            dfs(depth + 1, index + 2, s, current + '.' + s.substr(index, 2), answer);
        }

        if (index + 2 < s.length() && isValidSegment(s.substr(index, 3))) {
            dfs(depth + 1, index + 3, s, current + '.' + s.substr(index, 3), answer);
        }
    }

private:
    bool isValidSegment(const string& segment) {
        if (segment.length() > 3 || (segment.length() > 1 && segment[0] == '0')) return false;
        int value = stoi(segment);
        return value >= 0 && value <= 255;
    }
};
```