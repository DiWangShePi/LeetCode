# 424. Longest Repeating Character Replacement

### Description

You are given a string s and an integer k. You can choose any character of the string and change it to any other uppercase English character. You can perform this operation at most k times.

Return the length of the longest substring containing the same letter you can get after performing the above operations.

### Example

###### Example I

> Input: s = "ABAB", k = 2
> Output: 4
> Explanation: Replace the two 'A's with two 'B's or vice versa.

###### Example II

> Input: s = "AABABBA", k = 1
> Output: 4
> Explanation: Replace the one 'A' in the middle with 'B' and form "AABBBBA".
> The substring "BBBB" has the longest repeating letters, which is 4.
> There may exists other ways to achieve this answer too.

### Solution

参考：

> https://leetcode.cn/problems/longest-repeating-character-replacement/solutions/586648/ti-huan-hou-de-zui-chang-zhong-fu-zi-fu-eaacp/

这一题的官方解法是我此前见到的最详尽的

```c++
class Solution {
public:
    int characterReplacement(string s, int k) {
        vector<int> count(26, 0); 
        int left = 0;             
        int max_count = 0;       
        int max_length = 0;      
        
        for (int right = 0; right < s.size(); ++right) {
            count[s[right] - 'A']++; 
            max_count = max(max_count, count[s[right] - 'A']); 
            
            while (right - left + 1 - max_count > k) {
                count[s[left] - 'A']--;
                left++;               
            }

            max_length = max(max_length, right - left + 1);
        }
        
        return max_length;
    }
};
```
