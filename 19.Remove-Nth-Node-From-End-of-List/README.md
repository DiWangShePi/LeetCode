# 19. Remove Nth Node From End of List

### 题目描述

给定一个链表，删除链表的倒数第n个，再返回链表的头部

### 题目解析

假定链表程度为N，遍历链表至第N-(n+1)个，将该节点的下一个，指向下一个的下一个

但由于不知道链表的长度，因此可以在正常链表遍历中，增加指针指向当前指针的第前n+1个，当当前节点
为tail时（下一个为空），维护的指针即指向了我们需要的节点。

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
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode* dummy = new ListNode(0, head);
        ListNode* currentNode = dummy;
        ListNode* targetNode = dummy;

        for (int i = 0; i <= n; i++) {
            currentNode = currentNode->next;
        }

        while(currentNode != nullptr) {
            currentNode = currentNode->next;
            targetNode = targetNode->next;
        }

        ListNode* nodeToDelete = targetNode->next;
        targetNode->next = targetNode->next->next;

        ListNode* newHead = dummy->next;

        delete nodeToDelete;
        delete dummy;

        return newHead;
    }
};
```
