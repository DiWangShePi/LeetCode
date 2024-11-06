# 92. Reverse Linked List II

### Question Description

Given the head of a singly linked list and two integers left and right where left <= right, reverse the nodes of the list from position left to position right, and return the reversed list.

**Example: **

```
Input: head = [1,2,3,4,5], left = 2, right = 4
Output: [1,4,3,2,5]
```

```
Input: head = [5], left = 1, right = 1
Output: [5]
```

### Solution

遍历链表并用一个额外的列表保存，随后再重新遍历的时候按照要求串起来。

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
    ListNode* reverseBetween(ListNode* head, int left, int right) {
        if (left == right) return head;

        vector<ListNode*> preservedList;
        ListNode* dummy = new ListNode(0, head);
        preservedList.push_back(dummy);
        while(head != nullptr) {
            preservedList.push_back(head);
            head = head->next;
        }

        preservedList[left - 1]->next = preservedList[right];
        for (int i = right; i > left; i--) {
            preservedList[i]->next = preservedList[i - 1];
        }
        if (right == preservedList.size() - 1) preservedList[left]->next = nullptr;
        else preservedList[left]->next = preservedList[right + 1];

        return dummy->next;
    }
};
```