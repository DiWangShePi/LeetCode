# 24. Swap Nodes in Pairs

### 题目描述

给定一个单项链表，在不变更节点值的前提下，交换相邻的两个节点

**示例：**

![](./swap_ex1.jpg)

```
Input: head = [1,2,3,4]
Output: [2,1,4,3]
```

### 题目解析

维护四个节点，`head`，`node1`，`node2`，`tail`。

原有的顺序为，head指向node1，node1指向node2，node2指向tail。

变更为head指向node2，node2指向node1，node1指向tail。

更新head为node1，依次重新赋值node1，node2，tail

### 代码实现

###### c++

```c++
class Solution {
public:
    ListNode* swapPairs(ListNode* head) {
        if (head == nullptr || head->next == nullptr) {
            return head;
        }

        ListNode* dummy = new ListNode(0, head);
        ListNode* current = dummy;

        while (current->next != nullptr && current->next->next != nullptr) {
            ListNode* node1 = current->next;
            ListNode* node2 = node1->next;
            ListNode* tail = node2->next;

            // Swap nodes
            current->next = node2;
            node2->next = node1;
            node1->next = tail;

            // Move current pointer
            current = node1;
        }

        return dummy->next;
    }
};
```
