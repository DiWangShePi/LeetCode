# 206. Reverse Linked List

### Decsription

Given the head of a singly linked list, reverse the list, and return the reversed list.

### Solution

这道题限制很少，做起来灵活度很高。你可以将他们拿出来组成数组，再一个个重新排列。

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
    ListNode* reverseList(ListNode* head) {
        if (head == nullptr || head->next == nullptr) return head;

        vector<ListNode*> nodeList;
        while (head != nullptr) {
            nodeList.push_back(head);
            head = head->next;
        }

        for (int i = nodeList.size() - 1; i > 0; i--) {
            nodeList[i]->next = nodeList[i - 1];
        }
        nodeList[0]->next = nullptr;
        return nodeList.back();
    }
};
```
