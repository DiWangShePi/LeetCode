# 2269. Find the K-Beauty of a Number

### Description

The k-beauty of an integer num is defined as the number of substrings of num when it is read as a string that meet the following conditions:

- It has a length of k.
- It is a divisor of num.
Given integers num and k, return the k-beauty of num.

Note:

- Leading zeros are allowed.
- 0 is not a divisor of any value.
A substring is a contiguous sequence of characters in a string.

### Example

###### Example I

> Input: num = 240, k = 2
> Output: 2
> Explanation: The following are the substrings of num of length k:
> - "24" from "240": 24 is a divisor of 240.
> - "40" from "240": 40 is a divisor of 240.
> Therefore, the k-beauty is 2.

###### Example II

> Input: num = 430043, k = 2
> Output: 2
> Explanation: The following are the substrings of num of length k:
> - "43" from "430043": 43 is a divisor of 430043.
> - "30" from "430043": 30 is not a divisor of 430043.
> - "00" from "430043": 0 is not a divisor of 430043.
> - "04" from "430043": 4 is not a divisor of 430043.
> - "43" from "430043": 43 is a divisor of 430043.
> Therefore, the k-beauty is 2.

### Solution

枚举所有可能的字符串

```c++
class Solution {
public:
    int divisorSubstrings(int num, int k) {
        string s_num = to_string(num);
        int n = s_num.size();
        int an = 0;
        for (int i = 0; i <= n - k; i++) {
            int tmp = stoi(s_num.substr(i, k));
            if (tmp && num % tmp == 0) an++;
        }
        return an;
    }
};
```
