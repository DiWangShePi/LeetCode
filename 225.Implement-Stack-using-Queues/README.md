# 225. Implement Stack using Queues

### Description

Implement a last-in-first-out (LIFO) stack using only two queues. The implemented stack should support all the functions of a normal stack (push, top, pop, and empty).

Implement the MyStack class:

void push(int x) Pushes element x to the top of the stack.
int pop() Removes the element on the top of the stack and returns it.
int top() Returns the element on the top of the stack.
boolean empty() Returns true if the stack is empty, false otherwise.
Notes:

You must use only standard operations of a queue, which means that only push to back, peek/pop from front, size and is empty operations are valid.
Depending on your language, the queue may not be supported natively. You may simulate a queue using a list or deque (double-ended queue) as long as you use only a queue's standard operations.

### Solution

当队列中只有一个元素时，栈和队列没什么区别。当有两个元素时，我们希望队列中的元素是新的在前面。

因此，将一个按顺序倒出来，塞给另一个，就可以了。

### Implementation

###### c++

```c++
class MyStack {
public:
    queue<int> queue1;
    queue<int> queue2;

    MyStack() {

    }

    void push(int x) {
        queue2.push(x);
        while(!queue1.empty()) {
            queue2.push(queue1.front());
            queue1.pop();
        }
        swap(queue1, queue2);
    }
    
    int pop() {
        int r = queue1.front();
        queue1.pop();
        return r;
    }
    
    int top() {
        int r = queue1.front();
        return r;
    }
    
    bool empty() {
        return queue1.empty();
    }
};
```
