# 83. Remove Duplicates from Sorted List

### Question Description

Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.

**Example: **

```
Input: head = [1,1,2]
Output: [1,2]
```

```
Input: head = [1,1,2,3,3]
Output: [1,2,3]
```

### Question Solution

跟上一题类似，每次遍历时遇到不重复的即可放入

### Code Implemption

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
    ListNode* deleteDuplicates(ListNode* head) {
        if (head == nullptr || head->next == nullptr) return head;

        vector<ListNode*> result;

        ListNode* lastOne = head;
        result.push_back(head);
        head = head->next;

        while (head != nullptr) {
            if (head->val != lastOne->val) {
                result.push_back(head);
                lastOne = head;
            }
            head = head->next;
        }

        ListNode* an = result[0];
        for (int i = 1; i < result.size(); i++) {
            an->next = result[i];
            an = an->next;
        }
        an->next = nullptr;
        return result[0];
    }
};
```

