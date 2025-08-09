# 438. Find All Anagrams in a String

### Description

Given two strings s and p, return an array of all the start indices of p's anagrams in s. You may return the answer in any order.

### Example

###### Example I

> Input: s = "cbaebabacd", p = "abc"
> Output: [0,6]
> Explanation:
> The substring with start index = 0 is "cba", which is an anagram of "abc".
> The substring with start index = 6 is "bac", which is an anagram of "abc".

###### Example II

> Input: s = "abab", p = "ab"
> Output: [0,1,2]
> Explanation:
> The substring with start index = 0 is "ab", which is an anagram of "ab".
> The substring with start index = 1 is "ba", which is an anagram of "ab".
> The substring with start index = 2 is "ab", which is an anagram of "ab".

### Solution

我们还是先考虑暴力的做法：遍历字符串s，记录每一位上，每个字符出现的频率。随后枚举可能的每一段，用频率表相减得到这一段字符串中，字符的出现频率，与字符串p的出现频率进行对比。

```c++
class Solution {
public:
    vector<int> findAnagrams(string s, string p) {
        int n = s.size(), m = p.size();
        vector<array<int, 26>> dict(n);

        array<int, 26> temp{};
        for (int i = 0; i < n; i++) {
            temp[s[i] - 'a']++;
            dict[i] = temp;
        }
        array<int, 26> dict_p{};
        for (char c : p) dict_p[c - 'a']++;

        vector<int> an;
        for (int i = 0; i <= n - m; i++) {
            if (check(dict, i, i + m, dict_p)) an.push_back(i);
        }
        return an;
    }

private:
    bool check(const vector<array<int, 26>>& dict, int i, int j, const array<int, 26>& dict_an) {
        const array<int, 26>& dict_p = dict[j - 1];
        static array<int, 26> dict_s; 
        if (i == 0) {
            fill(dict_s.begin(), dict_s.end(), 0);
        } else {
            dict_s = dict[i - 1];
        }
        for (int k = 0; k < 26; k++) {
            if (dict_p[k] - dict_s[k] != dict_an[k]) return false;
        }
        return true;
    }
};
```

我们随后发现，遍历过程中，我们用到的实际只有代表长度为p.size()那么一段字符的字符表，因此我们不必为整个字符串s都维护一个字符频率表，可以采用滑动窗口的方式，增加新字符的频率，减少剔除出去的，再与目标字符串进行比较。

```c++
class Solution {
public:
    vector<int> findAnagrams(string s, string p) {
        int n = s.size(), m = p.size();
        if (m > n) return {};

        array<int, 26> target{}, window{};
        for (char c : p) target[c - 'a']++;

        vector<int> ans;
        for (int i = 0; i < n; i++) {
            window[s[i] - 'a']++;

            if (i >= m) {
                window[s[i - m] - 'a']--;
            }

            if (i >= m - 1 && window == target) {
                ans.push_back(i - m + 1);
            }
        }
        return ans;
    }
};
```

但我们随即发现，既然我们都已经知道每次变动的字符了，那么那些没有变动的字符，相比与此前的结果是一致的，不必再重复比较了

```c++
class Solution {
public:
    vector<int> findAnagrams(string s, string p) {
        int n = s.size(), m = p.size();
        if (m > n) return {};

        array<int, 26> count{};
        for (char c : p) count[c - 'a']++;  

        int diffCount = m; 
        vector<int> ans;

        for (int i = 0; i < n; i++) {
            if (--count[s[i] - 'a'] >= 0) diffCount--;
            if (i >= m && ++count[s[i - m] - 'a'] > 0) diffCount++;

            if (i >= m - 1 && diffCount == 0) ans.push_back(i - m + 1);
        }
        return ans;
    }
};
```
