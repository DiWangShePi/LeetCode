# 344. Reverse String

### Description

Write a function that reverses a string. The input string is given as an array of characters s.

You must do this by modifying the input array in-place with O(1) extra memory.

### Example 

###### Example I

```
Input: s = ["h","e","l","l","o"]
Output: ["o","l","l","e","h"]
```

###### Example II

```
Input: s = ["H","a","n","n","a","h"]
Output: ["h","a","n","n","a","H"]
```

### Solution

遍历一遍，交换字符顺序

```c++
class Solution {
public:
    void reverseString(vector<char>& s) {
        int i = 0, j = s.size() - 1;
        while (i <= j) {
            swap(s, i, j);
            i++; j--;
        }
    }

private:
    void swap(vector<char>& s, int i, int j) {
        char temp = s[i];
        s[i] = s[j];
        s[j] = temp;
    }
};
```
