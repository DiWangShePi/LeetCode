# 1475. Final Prices With a Special Discount in a Shop

**Tags:** Monotonic Stack

### Description

You are given an integer array prices where prices[i] is the price of the ith item in a shop.

There is a special discount for items in the shop. If you buy the ith item, then you will receive a discount equivalent to prices[j] where j is the minimum index such that j > i and prices[j] <= prices[i]. Otherwise, you will not receive any discount at all.

Return an integer array answer where answer[i] is the final price you will pay for the ith item of the shop, considering the special discount.

### Example

###### Example I

> Input: prices = [8,4,6,2,3]
> Output: [4,2,4,2,3]
> Explanation: 
> For item 0 with price[0]=8 you will receive a discount equivalent to prices[1]=4, therefore, the final price you will pay is 8 - 4 = 4.
> For item 1 with price[1]=4 you will receive a discount equivalent to prices[3]=2, therefore, the final price you will pay is 4 - 2 = 2.
> For item 2 with price[2]=6 you will receive a discount equivalent to prices[3]=2, therefore, the final price you will pay is 6 - 2 = 4.
> For items 3 and 4 you will not receive any discount at all.

###### Example II

> Input: prices = [1,2,3,4,5]
> Output: [1,2,3,4,5]
> Explanation: In this case, for all items, you will not receive any discount at all.

###### Example III

> Input: prices = [10,1,1,6]
> Output: [9,0,1,6]

### Solution

能暴力的时候，先让我暴力一下

```c++
class Solution {
public:
    vector<int> finalPrices(vector<int>& prices) {
        vector<int> an;
        int n = prices.size();
        for (int i = 0; i < n; i++) {
            int c = prices[i];
            an.push_back(c);
            for (int j = i + 1; j < n; j++) {
                if (prices[j] <= c) {
                    an.back() -= prices[j];
                    break;
                }
            }
        }
        return an;
    }
};
```

接下来我们考虑更优雅的做法。从尾部向前遍历，用一个单调栈记录递增的数字。如果当前元素大于栈顶元素，即存在折扣，计算折扣后 pop 直到只剩下大于的元素，随后加入栈中。

```c++
class Solution {
public:
    vector<int> finalPrices(vector<int>& prices) {
        int n = prices.size();
        vector<int> an(n);
        stack<int> s;
        for (int i = n - 1; i > -1; i--) {
            while (!s.empty() && s.top() > prices[i]) s.pop();
            an[i] = s.empty() ? prices[i] : prices[i] - s.top();
            s.push(prices[i]); 
        }
        return an;
    }
};
```
