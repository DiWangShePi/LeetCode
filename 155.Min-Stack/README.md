# 155. Min Stack

### Description

Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

Implement the MinStack class:

- MinStack() initializes the stack object.
- void push(int val) pushes the element val onto the stack.
- void pop() removes the element on the top of the stack.
- int top() gets the top element of the stack.
- int getMin() retrieves the minimum element in the stack.
You must implement a solution with O(1) time complexity for each function.

### Solution

前三个功能都很简单，第四个需要额外思考一下。

我们不妨从栈为空的时候开始。

- 如果我们加入任意一个元素，我们可以知道此时栈中最小的元素就是该元素。
- 如果我们再加入一个元素，此时栈中最小的元素应该是两个元素中，较小的哪一个。

以此类推，我们发现栈中的每一个元素可以与之对应一个最小值，该最小值就是该元素和上一个元素对应的最小值中较小的哪一个。

### Implementation

###### c++

```c++
class MinStack {
    stack<int> minStack;
    stack<int> helperStack;

public:
    MinStack() {
        helperStack.push(INT_MAX);
    }
    
    void push(int val) {
        minStack.push(val);
        helperStack.push(min(helperStack.top(), val));
    }
    
    void pop() {
        minStack.pop();
        helperStack.pop();
    }
    
    int top() {
        return minStack.top();
    }
    
    int getMin() {
        return helperStack.top();
    }
};
```
