# 482. License Key Formatting

### Descritpion

You are given a license key represented as a string s that consists of only alphanumeric characters and dashes. The string is separated into n + 1 groups by n dashes. You are also given an integer k.

We want to reformat the string s such that each group contains exactly k characters, except for the first group, which could be shorter than k but still must contain at least one character. Furthermore, there must be a dash inserted between two groups, and you should convert all lowercase letters to uppercase.

Return the reformatted license key.

### Example 

###### Example I

> Input: s = "5F3Z-2e-9-w", k = 4
> Output: "5F3Z-2E9W"
> Explanation: The string s has been split into two parts, each part has 4 characters.
> Note that the two extra dashes are not needed and can be removed.

###### Example II

> Input: s = "2-5g-3-J", k = 2
> Output: "2-5G-3J"
> Explanation: The string s has been split into three parts, each part has 2 characters except the first part as it could be shorter as mentioned above.

### Solution

由于第一个的长度不限，因此我们需要从后往前构造。

```c++
class Solution {
public:
    string licenseKeyFormatting(string s, int k) {
        int n = s.size();
        string ans;
        int count = 0;
        
        for (int i = n - 1; i >= 0; i--) {
            if (s[i] == '-') continue;
            
            ans.push_back(toupper(s[i]));
            count++;
            
            if (count % k == 0) {
                ans.push_back('-');
            }
        }
        
        if (!ans.empty() && ans.back() == '-') {
            ans.pop_back();
        }
        
        reverse(ans.begin(), ans.end());
        return ans;
    }
};
```
