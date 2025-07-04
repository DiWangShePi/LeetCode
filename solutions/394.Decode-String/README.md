# 394. Decode String

### Description

Given an encoded string, return its decoded string.

The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.

You may assume that the input string is always valid; there are no extra white spaces, square brackets are well-formed, etc. Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there will not be input like 3a or 2[4].

The test cases are generated so that the length of the output will never exceed 105.

### Example 

###### Example I

> Input: s = "3[a]2[bc]"
> Output: "aaabcbc"

###### Example II

> Input: s = "3[a2[c]]"
> Output: "accaccacc"

###### Example III

> Input: s = "2[abc]3[cd]ef"
> Output: "abcabccdcdcdef"

### Solution

递归，以`[`和`]`作为进入和退出递归的分隔符

```c++
class Solution {
public:
    string decodeString(string s) {
        int index = 0;
        return helper(s, index);
    }

private:
    string helper(string& s, int& index) {
        string result;
        
        while (index < s.size() && s[index] != ']') {
            if (!isdigit(s[index])) {
                result += s[index++];
            } else {
                // Parse number
                int times = 0;
                while (index < s.size() && isdigit(s[index])) {
                    times = times * 10 + (s[index++] - '0');
                }
                
                // Skip '['
                index++;
                
                // Recursively decode the substring
                string sub = helper(s, index);
                
                // Skip ']'
                index++;
                
                // Append the substring multiple times
                for (int i = 0; i < times; i++) {
                    result += sub;
                }
            }
        }
        
        return result;
    }
};
```