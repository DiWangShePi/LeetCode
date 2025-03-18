# 234. Palindrome Linked List

### Description

Given the head of a singly linked list, return true if it is a palindrome or false otherwise.

### Solution

遍历后加入数组，双指针移动检查值是否相符。

> 如果我将值保留下来，能算 O(1)的额外空间吗？

> 不能，空间会溢出。
> 逆天，你能保存一个这么长的地址数组，保存字符串就不行了？
> 原来是因为 to_string 太吃开销了。

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

> 更加优雅的链表反转，仅需要 O(1)的额外空间复杂度。但只适用于操作过程中别的线程不会访问该链表的状况，因为需要改动内部结构

```c++
class Solution {
public:
    bool isPalindrome(ListNode* head) {
        if (!head || !head->next) return true; 
        
        ListNode *slow = head, *fast = head, *prev = nullptr;

        while (fast && fast->next) {
            fast = fast->next->next;
            ListNode* temp = slow;
            slow = slow->next;
            temp->next = prev;
            prev = temp;
        }

        if (fast) slow = slow->next;
        
        while (slow) {
            if (slow->val != prev->val) return false;
            slow = slow->next;
            prev = prev->next;
        }
        
        return true;
    }
};
```