# 1019. Next Greater Node In Linked List

**Tags:** Array, Monotonic Stack

### Description

You are given the head of a linked list with n nodes.

For each node in the list, find the value of the next greater node. That is, for each node, find the value of the first node that is next to it and has a strictly larger value than it.

Return an integer array answer where answer[i] is the value of the next greater node of the ith node (1-indexed). If the ith node does not have a next greater node, set answer[i] = 0.

### Example

###### Example I

![](./linkedlistnext1.jpg)

> Input: head = [2,1,5]
> Output: [5,5,0]

###### Example II

![](./linkedlistnext2.jpg)

> Input: head = [2,7,4,3,5]
> Output: [7,0,5,5,0]

### Solution

遍历一遍链表，然后用倒过来遍历一遍，用单调栈记录。

```c++
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    vector<int> nextLargerNodes(ListNode* head) {
        vector<int> arr;
        while (head != nullptr) {
            arr.push_back(head->val);
            head = head->next;
        }

        int n = arr.size();
        vector<int> an(n, 0);
        stack<int> s;
        for (int i = n - 1; i > -1; i--) {
            while (!s.empty() && s.top() <= arr[i]) s.pop();
            if (!s.empty()) an[i] = s.top();
            s.push(arr[i]);
        }
        return an;
    }
};
```
