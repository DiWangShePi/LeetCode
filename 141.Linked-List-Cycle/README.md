# 141. Linked List Cycle

### Description

Given head, the head of a linked list, determine if the linked list has a cycle in it.

There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the next pointer. Internally, pos is used to denote the index of the node that tail's next pointer is connected to. Note that pos is not passed as a parameter.

Return true if there is a cycle in the linked list. Otherwise, return false.

### Solution

两个指针，一个一次走两步，一个一次走一步。如果走两步的在某一刻等于走一步的，那么就是有环，否则就是没有。

### Implementation

###### c++

```c++
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    bool hasCycle(ListNode *head) {
        if (!head || !head->next) return false;

        ListNode* fast = head->next;
        ListNode* slow = head;

        while (fast && fast->next) {
            if (slow == fast) return true;

            fast = fast->next->next;
            slow = slow->next;
        }
        return false;
    }
};
```