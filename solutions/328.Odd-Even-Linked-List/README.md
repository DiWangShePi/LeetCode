# 328. Odd Even Linked List

### Description

Given the head of a singly linked list, group all the nodes with odd indices together followed by the nodes with even indices, and return the reordered list.

The first node is considered odd, and the second node is even, and so on.

Note that the relative order inside both the even and odd groups should remain as it was in the input.

You must solve the problem in O(1) extra space complexity and O(n) time complexity.

### Example 

###### Example I

![](./oddeven-linked-list.jpg)

```
Input: head = [1,2,3,4,5]
Output: [1,3,5,2,4]
```

###### Example II

![](./oddeven2-linked-list.jpg)

```
Input: head = [2,1,3,5,6,4,7]
Output: [2,3,6,7,1,5,4]
```

### Solution

我一开始的想法是遍历的过程中一个一个将偶数的塞到末尾去，但这就会因为链表的总长度而带来两种结束判断情况：
1. 总长度为奇数，那么遍历到末尾的时候就是工作做完了，可以返回了。
2. 总长度为偶数，那么遍历到末尾的时候，工作还没有做完。

局部信息无法分辨这两种情况，必须要先遍历一遍得知了全局信息才能改。略显麻烦。

一种更优雅的方式是在遍历的过程中，将偶数的和奇数的分别保存在两条链表中，最终将两条链表合并即可。

> 这里讨论的都是O(1)空间复杂度和O(N)时间复杂度的解法

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
    ListNode* oddEvenList(ListNode* head) {
        if (head == nullptr || head->next == nullptr) return head;

        ListNode* odd = head;
        ListNode* even = head->next;
        ListNode* evenHead = even;

        while (even != nullptr && even->next != nullptr) {
            odd->next = even->next;     
            odd = odd->next;   

            even->next = odd->next;     
            even = even->next; 
        }
        odd->next = evenHead;

        return head;
    }
};
```

> 那么有没有比这个更粗劣的解法呢，有的兄弟有的

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
    ListNode* oddEvenList(ListNode* head) {
        if (head == nullptr || head->next == nullptr) return head;

        vector<ListNode*> array;
        while (head->next != nullptr) {
            array.push_back(head);
            head = head->next;
        }
        array.push_back(head);

        int n = array.size();
        for (int i = 2; i < n; i = i + 2) {
            array[i - 2]->next = array[i];
        }
        for (int i = 3; i < n; i = i + 2) {
            array[i - 2]->next = array[i];
        }

        if (n % 2 == 0) {
            array[n - 2]->next = array[1];
            array[n - 1]->next = nullptr;
        } else {
            array[n - 1]->next = array[1];
            array[n - 2]->next = nullptr;
        }
        return array[0];
    }
};
```

> 我觉得这个解法已经很不好了，但奇妙的是它的时间开销（在LeetCode给的排名上）还有更差的。