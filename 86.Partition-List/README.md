# 86. Partition List

### Question Description

Given the head of a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.

You should preserve the original relative order of the nodes in each of the two partitions.

```
Input: head = [1,4,3,2,5,2], x = 3
Output: [1,2,2,4,3,5]
```

```
Input: head = [2,1], x = 2
Output: [1,2]
```

### Question Solution

双指针，第一个指针指向大于n的第一个节点，第二个指针顺序遍历。
每遇到一个小于等于n的值就交换两个指针指向的节点的值。

### Code Implementation

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
    ListNode* partition(ListNode* head, int x) {
        if (head == nullptr) return nullptr;

        ListNode* dummy = new ListNode(0); 
        dummy->next = head;
        
        ListNode* pointer = dummy; 
        ListNode* current = head;
        ListNode* prev = dummy; 

        while (current != nullptr) {
            if (current->val < x) {
                if (pointer->next != current) {
                    prev->next = current->next;
                    current->next = pointer->next;
                    pointer->next = current;
                    pointer = pointer->next;
                    current = prev->next;
                } else {
                    pointer = pointer->next;
                    prev = current;
                    current = current->next;
                }
            } else {
                prev = current;
                current = current->next;
            }
        }

        ListNode* result = dummy->next;
        delete dummy; 
        return result;
    }
};
```