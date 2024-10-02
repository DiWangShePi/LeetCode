# 61. Rotate List

### 题目描述

给定一个链表，将链表向右旋转K次。

**示例：**

```
Input: head = [1,2,3,4,5], k = 2
Output: [4,5,1,2,3]
```

```
Input: head = [0,1,2], k = 4
Output: [2,0,1]
```

### 题目解析

创建数组，遍历链表并将节点依次放入到数组中。获取到链表长度后，用k模链表长度，再用链表长度减去得到的数字，设为m。

将链表从倒数第m个开始截断，重组链表，即可。

这样的方式会因为额外的数组声明而带来更大的开销，也可以遍历两次链表，一次获取长度，一次得到新链表头。

### 代码实现

###### c++

###### c++

using vector
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
    ListNode* rotateRight(ListNode* head, int k) {
        if (!head || k == 0) {
            return head;
        }
        
        std::vector<ListNode*> nodeList;
        ListNode* current = head;

        while (current != nullptr) {
            nodeList.push_back(current);
            current = current->next;
        }

        int n = nodeList.size();
        k = k % n;

        if (k == 0) {
            return head; 
        }

        int m = n - k;

        ListNode* newHead = nodeList[m];
        nodeList[m - 1]->next = nullptr; 
        nodeList[n - 1]->next = nodeList[0]; 

        return newHead;
    }
};
```

iterate twice
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
    
    ListNode* rotateRight(ListNode* head, int k) {
        //base case
        if(!head || !head->next|| k==0) return head;

        //find length of the ll
        int len = 1;
        ListNode* temp = head;
        while (temp->next) {
            len++;
            temp = temp->next;
        }
        temp->next = head;

        //check No. of rotation
        if(k % len == 0) return head;
        k = k % len;
        int p = len - k;
        
        //find the lastnode
        ListNode* lastnode = head;
        while(--p) lastnode = lastnode->next;
        ListNode* newhead = lastnode->next;
        lastnode->next = NULL;

        return newhead;
    }
};
```