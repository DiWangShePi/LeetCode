# 402. Remove K Digits

### Description

Given string num representing a non-negative integer num, and an integer k, return the smallest possible integer after removing k digits from num.

### Example 

###### Example I

> Input: num = "1432219", k = 3
> Output: "1219"
> Explanation: Remove the three digits 4, 3, and 2 to form the new number 1219 which is the smallest.

###### Example II

> Input: num = "10200", k = 1
> Output: "200"
> Explanation: Remove the leading 1 and the number is 200. Note that the output must not contain leading zeroes.

###### Example III

> Input: num = "10", k = 2
> Output: "0"
> Explanation: Remove all the digits from the number and it is left with nothing which is 0.

### Solution

先给出一个贪心的方法

```c++
class Solution {
public:
    string removeKdigits(string num, int k) {
        // do it in O(k * num.size())
        for (int i = 0; i < k; i++) helper(num);

        // remove leading zero
        int i = 0;
        while (num[i] == '0') i++;
        num = num.substr(i, num.size() - i);

        // check in case all been removed
        return num.size() == 0 ? "0" : num;
    }

private:
    void helper(string& num) {
        int i = 0;
        while (i + 1 < num.size() && num[i] <= num[i + 1]) {
            i++;
        }
        num.erase(i, 1); 
    }
};
```

再给一个优化的，用单调栈的做法：

```c++
class Solution {
public:
    string removeKdigits(string num, int k) {
        if (k == num.size()) return "0";

        vector<char> stack;
        for (char c : num) {
            while (!stack.empty() && k > 0 && stack.back() > c) {
                stack.pop_back();
                k--;
            }
            stack.push_back(c);
        }

        while (k > 0) {
            stack.pop_back();
            k--;
        }

        int i = 0;
        while (i < stack.size() && stack[i] == '0') i++;

        string an(stack.begin() + i, stack.end());
        return an.size() == 0 ? "0" : an;
    }
};
```
