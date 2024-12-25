# 142. Linked List Cycle II

### Description

Given the head of a linked list, return the node where the cycle begins. If there is no cycle, return null.

There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the next pointer. Internally, pos is used to denote the index of the node that tail's next pointer is connected to (0-indexed). It is -1 if there is no cycle. Note that pos is not passed as a parameter.

Do not modify the linked list.

### Solution

与上一题一样，哈希表可以解决这个问题。
我们遍历链表，将每一个遇到的节点和它的位置加入到哈希表中。
当某一步，我们发现下一个指针指向的节点曾经出现在哈希表中时，该节点的位置即为环入口。

当然，我们也可以考虑双指针的方式。
与上一题一样，我们采用一个走一步，一个走两步的两个指针。
假定从链表开头到环入口位置的长度为a，从环入口到两个指针第一次相遇的长度为b，从第一次相遇到结尾长度为c。
我们有：

$$
2 \cdot (a + b) = a + b + n \cdot (b + c)
$$

变化一下后得到：

$$
a = c + (n-1) \cdot (b + c)
$$

这意味着从该相遇点到入环点的距离，加上(n-1)倍的环长，等于链表开头到入环点的距离。

于是我们在此时新建一个指针，指向开头，与slow一样每次走一步，他们将在入环点的地方相遇。

### Implementation

###### c++

```c++
class Solution {
public:
    ListNode* detectCycle(ListNode* head) {
        if (!head || !head->next) return nullptr;

        ListNode *fast = head, *slow = head;
        while (fast && fast->next) {
            slow = slow->next;
            fast = fast->next->next;

            if (fast == slow) {
                fast = head;
                while (head != slow) {
                    slow = slow->next;
                    head = head->next;
                }
                return slow;
            }
        }
        return nullptr;
    }
};
```