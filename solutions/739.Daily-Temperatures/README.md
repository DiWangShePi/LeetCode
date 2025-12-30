# 739. Daily Temperatures

### Description

Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.

### Example

###### Example I

> Input: temperatures = [73,74,75,71,69,72,76,73]
> Output: [1,1,4,2,1,1,0,0]

###### Example II

> Input: temperatures = [30,40,50,60]
> Output: [1,1,1,0]

###### Example III

> Input: temperatures = [30,60,90]
> Output: [1,1,0]

### Solution

倒过来遍历，将得到的数字下标放在一个栈中。如果当前元素大于栈顶所代表的元素，则将栈顶元素出栈。最后将当前元素压入栈中。

```c++
class Solution {
public:
    vector<int> dailyTemperatures(vector<int>& temperatures) {
        int n = temperatures.size();
        vector<int> ans(n, 0);
        stack<int> st;

        for (int i = n - 1; i >= 0; --i) {
            while (!st.empty() && temperatures[st.top()] <= temperatures[i]) {
                st.pop();
            }
            if (!st.empty()) {
                ans[i] = st.top() - i;
            }
            st.push(i);
        }
        return ans;
    }
};
```
