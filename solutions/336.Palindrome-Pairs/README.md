# 336. Palindrome Pairs

### Description

You are given a 0-indexed array of unique strings words.

A palindrome pair is a pair of integers (i, j) such that:

- 0 <= i, j < words.length,
- i != j, and
- words[i] + words[j] (the concatenation of the two strings) is a palindrome.

Return an array of all the palindrome pairs of words.

You must write an algorithm with O(sum of words[i].length) runtime complexity.

### Example 

###### Example I

> Input: words = ["abcd","dcba","lls","s","sssll"]
> Output: [[0,1],[1,0],[3,2],[2,4]]
> Explanation: The palindromes are ["abcddcba","dcbaabcd","slls","llssssll"]

###### Example II

> Input: words = ["bat","tab","cat"]
> Output: [[0,1],[1,0]]
> Explanation: The palindromes are ["battab","tabbat"]


###### Example III

> Input: words = ["a",""]
> Output: [[0,1],[1,0]]
> Explanation: The palindromes are ["a","a"]

### Solution

我们先考虑纯粹暴力的做法：将字符串两两配对，检测是否为回文字符串。时间复杂度也是纯粹的O(N^2)的。
空间复杂度倒是不大，毕竟就传个字符串进去。

```c++
class Solution {
public:
    vector<vector<int>> palindromePairs(vector<string>& words) {
        int n = words.size();
        vector<vector<int>> an;
        if (n <= 1) return an;

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (i == j) continue;
                string current = words[i] + words[j];
                if (check(current, 0, current.size() - 1)) an.push_back({i, j});
            }
        }
        return an;
    }

private:
    bool check(const string& s, int left, int right) {
        while (left < right) {
            if (s[left++] != s[right--]) return false;
        }
        return true;
    }
};
```

接下来我们考虑时间上优雅一点，空间上笨拙一些的解法。

用一个字典存储每个遍历过的字符串和它所需要的能构成回文字符串的另一个字符串。
遍历字符串的时候，尝试每一种可能的切分，将字符串划为left和right两个部分。

- 如果right是回文的，检查reversed(left)是否存在
- 如果left是回文的，检查reversed(right)是否存在。

```c++
class Solution {
public:
    vector<vector<int>> palindromePairs(vector<string>& words) {
        unordered_map<string, int> reversed_map;
        vector<vector<int>> result;

        // Step 1: Store reversed strings in map
        for (int i = 0; i < words.size(); ++i) {
            string reversed = words[i];
            reverse(reversed.begin(), reversed.end());
            reversed_map[reversed] = i;
        }

        // Step 2: For each word, try all cuts
        for (int i = 0; i < words.size(); ++i) {
            string word = words[i];
            int len = word.length();
            for (int cut = 0; cut <= len; ++cut) {
                string left = word.substr(0, cut);
                string right = word.substr(cut);

                // Case 1: left is palindrome, look for reversed right
                if (check(word, 0, cut - 1)) {
                    auto it = reversed_map.find(right);
                    if (it != reversed_map.end() && it->second != i) {
                        result.push_back({it->second, i});
                    }
                }

                // Case 2: right is palindrome, look for reversed left
                // skip cut == len to avoid duplicates
                if (cut != len && check(word, cut, len - 1)) {
                    auto it = reversed_map.find(left);
                    if (it != reversed_map.end() && it->second != i) {
                        result.push_back({i, it->second});
                    }
                }
            }
        }

        return result;
    }

private:
    bool check(const string& s, int left, int right) {
        while (left < right) {
            if (s[left++] != s[right--]) return false;
        }
        return true;
    }
};
```
