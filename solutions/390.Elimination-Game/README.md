# 390. Elimination Game

### Description

You have a list arr of all integers in the range [1, n] sorted in a strictly increasing order. Apply the following algorithm on arr:

- Starting from left to right, remove the first number and every other number afterward until you reach the end of the list.
- Repeat the previous step again, but this time from right to left, remove the rightmost number and every other number from the remaining numbers.
- Keep repeating the steps again, alternating left to right and right to left, until a single number remains.

Given the integer n, return the last number that remains in arr.

### Example 

###### Example I

```
Input: n = 9
Output: 6
Explanation:
arr = [1, 2, 3, 4, 5, 6, 7, 8, 9]
arr = [2, 4, 6, 8]
arr = [2, 6]
arr = [6]
```

###### Example II

```
Input: n = 1
Output: 1
```

### Solution

先尝试暴力的方式：

```c++
class Solution {
public:
    int lastRemaining(int n) {
        vector<int> array;
        for (int i = 1; i <= n; i++) array.push_back(i);

        return helper(array, 1);
    }

private:
    int helper(vector<int> array, int towards) {
        int n = array.size();
        if (n == 1) return array[0];

        vector<int> current;
        if (towards == 1) {
            for (int i = 1; i < n; i = i + 2) current.push_back(array[i]);
        } else {
            for (int i = n - 2; i > -1; i = i - 2) current.push_back(array[i]);
            sort(current.begin(), current.end());
        }
        return helper(current, -towards);
    }
};
```

我们也可以采用模拟的方式，避免真的构造数组。每一次追踪新数组的开头的数字，这一数字只会在两种情况下变更：

1. 当前是从左往右遍历。
2. 从右往左遍历，且数组长度为奇数。

变更的长度即为步长（即此时保留的数组的下一个数字）。

```c++
class Solution {
public:
    int lastRemaining(int n) {
        int head = 1;             
        int step = 1;            
        bool leftToRight = true;  
        int remaining = n;        

        while (remaining > 1) {
            if (leftToRight || remaining % 2 == 1) {
                head += step;
            }
            step *= 2;             
            remaining /= 2;         
            leftToRight = !leftToRight; 
        }
        return head;
    }
};
```
