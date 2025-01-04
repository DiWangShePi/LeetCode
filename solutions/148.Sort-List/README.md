# 148. Sort List

### Description

Given the head of a linked list, return the list after sorting it in ascending order.

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
    ListNode* sortList(ListNode* head) {
        ListNode* dummy = new ListNode(0, head);
        vector<int> value;
        while (head != nullptr) {
            value.push_back(head->val);
            head = head->next;
        }

        sort(value.begin(), value.end());

        head = dummy->next;
        for (int i = 0; i < value.size(); i++) {
            head->val = value[i];
            head = head->next;
        }
        return dummy->next;
    }
};
```
