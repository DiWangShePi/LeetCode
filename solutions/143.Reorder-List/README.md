# 143. Reorder List

### Description

You are given the head of a singly linked-list. The list can be represented as:

> L0 → L1 → … → Ln - 1 → Ln
Reorder the list to be on the following form:

> L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
You may not modify the values in the list's nodes. Only nodes themselves may be changed.

### Solution

一个列表，遍历保存所有元素，然后重新排列。

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
    void reorderList(ListNode* head) {
        if (!head || !head->next) return;

        vector<ListNode*> node_list;
        while (head) {
            node_list.push_back(head);
            head = head->next;
        }

        int i = 0;
        for ( ; i < node_list.size() / 2; i++) {
            if (i - 1 > -1) {
                node_list[node_list.size() - 1 - (i - 1)]->next = node_list[i];
            }
            node_list[i]->next = node_list[node_list.size() - 1 - i];
        }
        if (node_list.size() % 2 != 0) {
            node_list[node_list.size() - 1 - (i - 1)]->next = node_list[i];
            node_list[i]->next = nullptr;
        } else {
            node_list[node_list.size() - 1 - (i - 1)]->next = nullptr;
        }

        return;
    }
};
```