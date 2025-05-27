# 367. Valid Perfect Square

### Description

Given a positive integer num, return true if num is a perfect square or false otherwise.

A perfect square is an integer that is the square of an integer. In other words, it is the product of some integer with itself.

You must not use any built-in library function, such as sqrt.

### Example 

###### Example I

```
Input: num = 16
Output: true
Explanation: We return true because 4 * 4 = 16 and 4 is an integer.
```

###### Example II

```
Input: num = 14
Output: false
Explanation: We return false because 3.742 * 3.742 = 14 and 3.742 is not an integer.
```

### Solution

不让用`sqrt`，那就暴力遍历吧

```c++
class Solution {
public:
    bool isPerfectSquare(int num) {
        for (long long i = 1; i <= num; i++) if (i * i == num) return true;
        return false;
    }
};
```

或者二分

```c++
class Solution {
public:
    bool isPerfectSquare(int num) {
        if (num < 1) return false;

        long long l = 0, r = num, mid, pro;
        while (l <= r) {
            mid = l + (r - l) / 2;
            pro = mid * mid;

            if (pro == num) return true;
            else if (pro > num) r = mid - 1;
            else l = mid + 1;
        }

        return false;
    }
};
```
