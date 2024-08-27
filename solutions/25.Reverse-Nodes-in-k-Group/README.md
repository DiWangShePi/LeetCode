# 25. Reverse Nodes in k-Group

### 题目描述

给定一个链表的头部，每次反转链表 k 个节点，并返回修改后的链表。

k 是一个正整数，小于或等于链表的长度。如果节点数不是 k 的倍数，则最后留下的节点应保持原样。

您不能更改列表节点中的值，只能更改节点本身。

**示例：**

```
Input: head = [1,2,3,4,5], k = 2
Output: [2,1,4,3,5]
```
![](./reverse_ex1.jpg)


```
Input: head = [1,2,3,4,5], k = 3
Output: [3,2,1,4,5]
```
![](./reverse_ex2.jpg)

### 题目解析

我们可以在前一题的解法上扩展，将维护的两个链表变为一个长度为k的链表组。

### 代码实现

###### c++

```c++
#include <vector>

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
    ListNode* reverseKGroup(ListNode* head, int k) {
        ListNode* dummy = new ListNode(0, head);
        ListNode* currentNode = dummy;

        std::vector<ListNode*> nodeList(k);
        while (checkLegal(currentNode->next, k)) {
            ListNode* prv = currentNode;
            for (int i = 0; i < k; i++) {
                nodeList[i] = currentNode->next;
                currentNode = currentNode->next;
            }
            ListNode* tail = currentNode->next;

            // Reverse the nodes in nodeList
            for (int i = k-1; i > 0; i--) {
                nodeList[i]->next = nodeList[i-1];
            }
            prv->next = nodeList[k-1];
            nodeList[0]->next = tail;

            currentNode = nodeList[0]; // Update currentNode for the next iteration
        }

        return dummy->next;
    }

private:
    bool checkLegal(ListNode* head, int k) {
        for (int i = 0; i < k; i++) {
            if (head == nullptr) {
                return false;
            }
            head = head->next;
        }
        return true;
    }
};
```