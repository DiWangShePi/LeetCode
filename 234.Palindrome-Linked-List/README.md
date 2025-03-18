# 234. Palindrome Linked List

### Description

Given the head of a singly linked list, return true if it is a palindrome or false otherwise.

### Solution

遍历后加入数组，双指针移动检查值是否相符。

> 如果我将值保留下来，能算O(1)的额外空间吗？

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
    bool isPalindrome(ListNode* head) {
        vector<ListNode*> nodeList;
        while (head != nullptr) {
            nodeList.push_back(head);
            head = head->next;
        }

        for (int i = 0; i < nodeList.size(); i++) {
            if (i >= nodeList.size() - i - 1) break; 
            if (nodeList[i]->val != nodeList[nodeList.size() - i - 1]->val) return false;
        }
        return true;
    }
};
```
