# 409. Longest Palindrome

### Description

Given a string s which consists of lowercase or uppercase letters, return the length of the longest palindrome that can be built with those letters.

Letters are case sensitive, for example, "Aa" is not considered a palindrome.

### Example 

###### Example I

> Input: s = "abccccdd"
> Output: 7
> Explanation: One longest palindrome that can be built is "dccaccd", whose length is 7.

###### Example II

> Input: s = "a"
> Output: 1
> Explanation: The longest palindrome that can be built is "a", whose length is 1.

### Solution

所有出现偶数次数的字符都可以全用，所有出现奇数次数的字符都可以减一个后用。如果存在出现奇数次数的字符，任选一个放在中间。

注意这一题里面大小写要区分开来。

```c++
class Solution {
public:
    int longestPalindrome(string s) {
        vector<int> freq(58, 0);
        for (char c : s) freq[c - 'A']++;
        
        int length = 0;
        bool hasOdd = false;
        for (int cnt : freq) {
            length += cnt / 2 * 2;
            if (cnt % 2 == 1) hasOdd = true;
        }
        return length + (hasOdd ? 1 : 0);
    }
};
```
