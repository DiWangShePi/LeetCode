# 203. Remove Linked List Elements

### Description

Given the head of a linked list and an integer val, remove all the nodes of the linked list that has Node.val == val, and return the new head.

### Solution

按要求实现即可

### Implementation

###### c++

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
    ListNode* removeElements(ListNode* head, int val) {
        ListNode* dummy = new ListNode(0, head);
        ListNode* currentNode = dummy;
        ListNode* nextNode = head;

        while (nextNode != nullptr) {
            if (nextNode->val == val) {
                currentNode->next = nextNode->next;
            } else {
                currentNode = currentNode->next;
            }
            nextNode = nextNode->next;
        }
        return dummy->next;
    }
};
```
