# 395. Longest Substring with At Least K Repeating Characters

### Description

Given a string s and an integer k, return the length of the longest substring of s such that the frequency of each character in this substring is greater than or equal to k.

if no such substring exists, return 0.

### Example 

###### Example I

> Input: s = "aaabb", k = 3
> Output: 3
> Explanation: The longest substring is "aaa", as 'a' is repeated 3 times.

###### Example II

> Input: s = "ababbc", k = 2
> Output: 5
> Explanation: The longest substring is "ababb", as 'a' is repeated 2 times and 'b' is repeated 3 times.

### Solution

我们先给出一个暴力的解法：遍历字符串，建立从0到每个字符构成的子字符串的字符及个数，随后枚举每种可能的子字符串（任意i和j），用相减的方式获得该子字符串的字符及个数以进行检查。

```c++
class Solution {
public:
    int longestSubstring(string s, int k) {
        int n = s.size();
        if (n < k) return 0;

        vector<int> dict(26, 0);
        vector<vector<int>> dict_list(n + 1);
        dict_list[0] = dict;
        for (int i = 1; i <= n; i++) {
            dict[s[i - 1] - 'a']++;
            dict_list[i] = dict;
        }

        int an = 0;
        for (int i = 1; i <= n; i++) {
            for (int j = 1; j <= n; j++) {
                if (check(dict_list[i - 1], dict_list[j], k)) an = max(an, j - i + 1);
            }
        }
        return an;
    }

private:
    bool check(vector<int> dict1, vector<int> dict2, int k) {
        for (int i = 0; i < 26; i++) {
            if (dict2[i] - dict1[i] != 0 && dict2[i] - dict1[i] < k) return false;
        }
        return true;
    }
};
```

> 按上面那样写会超时，同思路优化一下可以过，但依然很慢

```c++
class Solution {
public:
    int longestSubstring(string s, int k) {
        int n = s.size();
        if (n < k) return 0;

        vector<vector<int>> dict_list(n + 1, vector<int>(26, 0));
        for (int i = 1; i <= n; i++) {
            dict_list[i] = dict_list[i - 1];
            dict_list[i][s[i - 1] - 'a']++;
        }

        int ans = 0;
        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j <= n; j++) {
                if (isValid(dict_list[i], dict_list[j], k))
                    ans = max(ans, j - i);
            }
        }
        return ans;
    }

private:
    bool isValid(const vector<int>& left, const vector<int>& right, int k) {
        for (int i = 0; i < 26; ++i) {
            int freq = right[i] - left[i];
            if (freq > 0 && freq < k) return false;
        }
        return true;
    }
};
```

现在我们考虑在思路上更优的解法：我们知道，如果某一个字符出现的次数（在整个字符串中）都不超过k，那么任何包含这个字符的子字符串都不可能满足要求。可能满足要求的，只能是在这些字符分割出的，更小的子字符串中出现。

```c++
class Solution {
public:
    int longestSubstring(string s, int k) {
        return helper(s, 0, s.size(), k);
    }

private:
    int helper(const string& s, int l, int r, int k) {
        if (r - l < k) return 0;

        vector<int> count(26, 0);
        for (int i = l; i < r; i++) count[s[i] - 'a']++;

        for (int i = l; i < r; i++) {
            if (count[s[i] - 'a'] < k) {
                int j = i + 1;
                while (j < r && count[s[j] - 'a'] < k) j++;
                return max(helper(s, l, i, k), helper(s, j, r, k));
            }
        }
        return r - l;
    }
};
```
