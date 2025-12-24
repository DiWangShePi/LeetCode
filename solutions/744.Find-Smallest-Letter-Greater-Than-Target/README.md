# 744. Find Smallest Letter Greater Than Target

**Tags:** Binary Search

### Description

You are given an array of characters letters that is sorted in non-decreasing order, and a character target. There are at least two different characters in letters.

Return the smallest character in letters that is lexicographically greater than target. If such a character does not exist, return the first character in letters.

### Example

###### Example I

> Input: letters = ["c","f","j"], target = "a"
> Output: "c"
> Explanation: The smallest character that is lexicographically greater than 'a' in letters is 'c'.

###### Example II

> Input: letters = ["c","f","j"], target = "c"
> Output: "f"
> Explanation: The smallest character that is lexicographically greater than 'c' in letters is 'f'.

###### Example III

> Input: letters = ["x","x","y","y"], target = "z"
> Output: "x"
> Explanation: There are no characters in letters that is lexicographically greater than 'z' so we return letters[0].

### Solution

二分查找

```c++
class Solution {
public:
    char nextGreatestLetter(vector<char>& letters, char target) {
        int l = 0, r = letters.size() - 1;
        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (letters[mid] > target) r = mid - 1;
            else l = mid + 1;
        }
        return l >= letters.size() ? letters[0] : letters[l];
    }
};
```
