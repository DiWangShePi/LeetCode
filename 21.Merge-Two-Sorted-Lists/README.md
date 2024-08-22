# 21. Merge Two Sorted Lists

### 题目描述

给定两个排好序的链表`List1`和`List2`的头部，将两个链表合并为一个排好序的链表，并返回该链表的头部。

### 题目解析

依次遍历两个链表，创建第三个链表并将前两个链表中较小的一个加入到新的链表中。

### 代码实现

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
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        ListNode* dummy = new ListNode(0, nullptr);
        ListNode* currentNode = dummy;
        while (list1 != nullptr && list2 != nullptr) {
            ListNode* newNode = new ListNode(0, nullptr);
            currentNode->next = newNode;
            if (list1->val <= list2->val) {
                newNode->val = list1->val;
                list1 = list1->next;
            } else {
                newNode->val = list2->val;
                list2 = list2->next;
            }
            currentNode = currentNode->next;
        }
        while (list1 != nullptr) {
            ListNode* newNode = new ListNode(list1->val, nullptr);
            
            list1 = list1->next;
            currentNode->next = newNode;
            currentNode = currentNode->next;
        }
        while (list2 != nullptr) {
            ListNode* newNode = new ListNode(list2->val, nullptr);
            
            list2 = list2->next;
            currentNode->next = newNode;
            currentNode = currentNode->next;
        }

        return dummy->next;
    }
};
```