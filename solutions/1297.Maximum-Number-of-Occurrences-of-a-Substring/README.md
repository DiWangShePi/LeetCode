# 1297. Maximum Number of Occurrences of a Substring

**Tags:** Sliding Window

### Description

Given a string s, return the maximum number of occurrences of any substring under the following rules:

- The number of unique characters in the substring must be less than or equal to maxLetters.
- The substring size must be between minSize and maxSize inclusive.

### Example 

###### Example I

> Input: s = "aababcaab", maxLetters = 2, minSize = 3, maxSize = 4
> Output: 2
> Explanation: Substring "aab" has 2 occurrences in the original string.
> It satisfies the conditions, 2 unique letters and size 3 (between minSize and maxSize).

###### Example II

> Input: s = "aaaa", maxLetters = 1, minSize = 3, maxSize = 3
> Output: 2
> Explanation: Substring "aaa" occur 2 times in the string. It can overlap.

### Solution

其实只需要按 MinSize 做滑动窗口，因为包含的字符多了，满足需求的可能性降低了

```c++
class Solution {
public:
    int maxFreq(string s, int maxLetters, int minSize, int maxSize) {
        unordered_map<string, int> an;
        for (int size = minSize; size <= minSize; size++) {
            unordered_map<int, int> dict;

            int j = 0;
            for ( ; j < size; j++){
                if (dict.count(s[j] - 'a') == 0) dict[s[j] - 'a'] = 1;
                else dict[s[j] - 'a']++;
            }
            if (dict.size() <= maxLetters) an[s.substr(0, size)] = 1;

            for ( ; j < s.size(); j++) {
                if (dict.count(s[j] - 'a') == 0) dict[s[j] - 'a'] = 1;
                else dict[s[j] - 'a']++;

                dict[s[j - size] - 'a']--;
                if (dict[s[j - size] - 'a'] == 0) dict.erase(s[j - size] - 'a');

                if (dict.size() <= maxLetters) {
                    if (an.count(s.substr(j - size + 1, size)) == 0) an[s.substr(j - size + 1, size)] = 1;
                    else an[s.substr(j - size + 1, size)]++;
                }
            }
        }
        
        int result = 0;
        for (auto it = an.begin(); it != an.end(); it++)
            result = max(result, it->second);
        return result;
    }
};
```
