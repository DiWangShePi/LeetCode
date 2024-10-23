# 82. Remove Duplicates from Sorted List II

### Question Description

Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list. Return the linked list sorted as well.

**Example: **

```
Input: head = [1,2,3,3,4,4,5]
Output: [1,2,5]
```

```
Input: head = [1,1,1,2,3]
Output: [2,3]
```

### Question Solution

遍历链表，保留一个指针指向节点，一个指示是否重复。若当前元素等于指针，则表示重复了，若不等于，检查是否重复，重复则直接更新，不重复则放弃。

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
    ListNode* deleteDuplicates(ListNode* head) {
        vector<ListNode*> result;
        ListNode* current = head;
        bool duplicate = false;


        if (head == nullptr || head->next == nullptr) return head;

        head = head->next;
        while (head != nullptr) {
            if (current->val == head->val) duplicate = true;
            else {
                if (!duplicate) {
                    result.push_back(current);
                }
                current = head;
                duplicate = false;
            }
            head = head->next;
        }
        if (!duplicate) result.push_back(current);

        if (result.size() == 0) return nullptr;

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
