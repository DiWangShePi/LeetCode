# 23. Merge k Sorted Lists

### 题目描述

给定K个排序好了的链表，将他们合并为一个链表并返回。

### 题目解析

采用堆排序，将所有的数字一个一个塞到堆里，再一个个取出来作为结果。该方式的时间复杂度是N*log(K)。如果沿用此前合并两个链表的方式，该方式的时间复杂度是KN。

### 代码实现

###### c++
```c++
class Solution {
public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        using NodePair = pair<int, ListNode*>;
        priority_queue<NodePair, vector<NodePair>, greater<NodePair>> pq;

        for (auto list : lists) {
            if (list) {
                pq.emplace(list->val, list);
            }
        }

        ListNode* dummy = new ListNode(-1);
        ListNode* current = dummy;

        while (!pq.empty()) {
            auto [val, node] = pq.top();
            pq.pop();
            current->next = node;
            current = current->next;
            if (node->next) {
                pq.emplace(node->next->val, node->next);
            }
        }

        return dummy->next;
    }
};
```