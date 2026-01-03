# 901. Online Stock Span

**Tags:** Monotonic Stack

### Description

Design an algorithm that collects daily price quotes for some stock and returns the span of that stock's price for the current day.

The span of the stock's price in one day is the maximum number of consecutive days (starting from that day and going backward) for which the stock price was less than or equal to the price of that day.

- For example, if the prices of the stock in the last four days is [7,2,1,2] and the price of the stock today is 2, then the span of today is 4 because starting from today, the price of the stock was less than or equal 2 for 4 consecutive days.
- Also, if the prices of the stock in the last four days is [7,34,1,2] and the price of the stock today is 8, then the span of today is 3 because starting from today, the price of the stock was less than or equal 8 for 3 consecutive days.
Implement the StockSpanner class:

- StockSpanner() Initializes the object of the class.
- int next(int price) Returns the span of the stock's price given that today's price is price.

### Example

###### Example I

> Input
> ["StockSpanner", "next", "next", "next", "next", "next", "next", "next"]
> [[], [100], [80], [60], [70], [60], [75], [85]]
> Output
> [null, 1, 1, 1, 2, 1, 4, 6]
> Explanation
> StockSpanner stockSpanner = new StockSpanner();
> stockSpanner.next(100); // return 1
> stockSpanner.next(80);  // return 1
> stockSpanner.next(60);  // return 1
> stockSpanner.next(70);  // return 2
> stockSpanner.next(60);  // return 1
> stockSpanner.next(75);  // return 4, because the last 4 prices (including today's price of 75) were less than or equal to today's price.
> stockSpanner.next(85);  // return 6

### Solution

先给一个暴力的做法：保存全部的列表，每一次往回遍历，找满足条件的序列。

```c++
class StockSpanner {
    vector<int> s;
public:
    StockSpanner() {}
    
    int next(int price) {
        s.push_back(price);
        int n = s.size(), an = 0;
        for (int i = n - 1; i > -1; i--) {
            if (s[i] <= price) an++;
            else break;
        }
        return an;
    }
};

/**
 * Your StockSpanner object will be instantiated and called as such:
 * StockSpanner* obj = new StockSpanner();
 * int param_1 = obj->next(price);
 */
```

或者也可以用单调栈，维护一个逐步递减的栈。如果当前元素小于目标值，就pop到大于，中间这一段就全是符合要求的值。

```c++
class StockSpanner {
    vector<int> p;
    stack<int> s;
public:
    StockSpanner() {}
    
    int next(int price) {
        p.push_back(price);
        while (!s.empty() && p[s.top()] <= price) s.pop();
        
        int an = p.size();
        an -= s.empty() ? 0 : s.top() + 1;
        s.push(p.size() - 1);
        return an;
    }
};

/**
 * Your StockSpanner object will be instantiated and called as such:
 * StockSpanner* obj = new StockSpanner();
 * int param_1 = obj->next(price);
 */
```