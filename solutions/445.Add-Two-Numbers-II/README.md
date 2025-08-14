# 445. Add Two Numbers II

**Tags:** Linked List, Math

### Description

You are given two non-empty linked lists representing two non-negative integers. The most significant digit comes first and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

### Example 

###### Example I

![](./sumii-linked-list.jpg)

> Input: l1 = [7,2,4,3], l2 = [5,6,4]
> Output: [7,8,0,7]

###### Example II

> Input: l1 = [2,4,3], l2 = [5,6,4]
> Output: [8,0,7]

###### Example III

> Input: l1 = [0], l2 = [0]
> Output: [0]

### Solution

遍历两个链表，将节点存放在数组中，然后倒过来相加求和。

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
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        vector<ListNode*> list1, list2;
        ListNode* current = l1;
        while (current) {
            list1.push_back(current);
            current = current->next;
        }
        current = l2;
        while (current) {
            list2.push_back(current);
            current = current->next;
        }

        int n = list1.size(), m = list2.size();
        if (m > n) return addTwoNumbers(l2, l1);

        int t = n - m, i = n - 1, carry = 0;
        for ( ; i - t >= 0; i--) {
            int c = list1[i]->val + list2[i - t]->val + carry;
            carry = c / 10;
            list1[i]->val = c % 10;
        }

        while (i >= 0 && carry != 0) {
            int c = list1[i]->val + carry;
            carry = c / 10;
            list1[i]->val = c % 10;

            i--;
        } 
        
        if (carry != 0) {
            ListNode* head = new ListNode(carry);
            head->next = l1;
            return head;
        } else return l1;
    }
};
```

更省空间的做法是原地翻转或者递归对齐，这里给出第二种实现，因为题目要求不要翻转。

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
    int getLength(ListNode* head) {
        int len = 0;
        while (head) {
            len++;
            head = head->next;
        }
        return len;
    }

    int addLists(ListNode* l1, ListNode* l2, int diff) {
        if (!l1) return 0;

        int sum;
        if (diff > 0) {  
            sum = l1->val + addLists(l1->next, l2, diff - 1);
        } else {  
            sum = l1->val + l2->val + addLists(l1->next, l2->next, 0);
        }

        l1->val = sum % 10;
        return sum / 10; 
    }

    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        int len1 = getLength(l1);
        int len2 = getLength(l2);

        if (len2 > len1) {
            swap(l1, l2);
            swap(len1, len2);
        }

        int carry = addLists(l1, l2, len1 - len2);

        if (carry) {
            ListNode* newHead = new ListNode(carry);
            newHead->next = l1;
            return newHead;
        }
        return l1;
    }
};
```
