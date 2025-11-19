# 1888. Minimum Number of Flips to Make the Binary String Alternating

**Tags:** Sliding Window

### Description

ou are given a binary string s. You are allowed to perform two types of operations on the string in any sequence:

- Type-1: Remove the character at the start of the string s and append it to the end of the string.
- Type-2: Pick any character in s and flip its value, i.e., if its value is '0' it becomes '1' and vice-versa.
Return the minimum number of type-2 operations you need to perform such that s becomes alternating.

The string is called alternating if no two adjacent characters are equal.

- For example, the strings "010" and "1010" are alternating, while the string "0100" is not.

### Example

###### Example I

> Input: s = "111000"
> Output: 2
> Explanation: Use the first operation two times to make s = "100011".
> Then, use the second operation on the third and sixth elements to make s = "101010".

###### Example II

> Input: s = "010"
> Output: 0
> Explanation: The string is already alternating.

###### Example III

> Input: s = "1110"
> Output: 1
> Explanation: Use the second operation on the second element to make s = "1010".
 
### Solution

题目不限制我们做操作一的次数，但操作一可能的结果本身是有限的。
每一次我们只能从开头拿走一个数字，再放到结尾，这就像是一个字符串的循环。我们会遇到最多 N 种可能。

我们因此可以将这个字符串加倍后拼在一起，从中取一个长度为 N 的字符串，即为操作一结束后的结果。

接下来，我们考虑操作二的影响。

我们发现，对于 0101 这样的字符串，每个字符代表的数字等于该字符的下标模 2 的结果。
统计符合要求的字符结果，设为 X，进行 N - X 次翻转即可将整个字符串变为 0101... 的形式。

对于不满足条件的字符来说，它对应的其实是 1010... 这样的形式。也就是进行 X 次翻转，可以将整个字符串变为 1010... 的形式。

```c++
class Solution {
public:
    int minFlips(string s) {
        int count = 0, n = s.size();

        int i = 0;
        for ( ; i < n; i++) {
            if ((s[i % n] - '0') == i % 2) 
                count++;
        }
        int ans = min(count, n - count);
        for ( ; i < 2 * n - 1; i++) {
            if ((s[i % n] - '0') == i % 2) 
                count++;
            if ((s[(i - n) % n] - '0') == (i - n) % 2)
                count--;
                
            ans = min({ans, count, n - count});
        }
        return ans;
    }
};
```
