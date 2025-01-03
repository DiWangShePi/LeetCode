# 147. Insertion Sort List

### Description

Given the head of a singly linked list, sort the list using insertion sort, and return the sorted list's head.

The steps of the insertion sort algorithm:

1. Insertion sort iterates, consuming one input element each repetition and growing a sorted output list.
2. At each iteration, insertion sort removes one element from the input data, finds the location it belongs within the sorted list and inserts it there.
3. It repeats until no input elements remain.
The following is a graphical example of the insertion sort algorithm. The partially sorted list (black) initially contains only the first element in the list. One element (red) is removed from the input data and inserted in-place into the sorted list with each iteration.

### Solution

冒泡排序应该就可以了

但由于指针是指向下一个的，需要反着的冒泡

### Implementation

###### c++

> 虽然将ListNode放到一个列表里，然后根据值sort一遍，然后重新排列是一个更简单且更快的方式。但正如莎士比亚曾经说的：
> "But what's the point?"

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
    ListNode* insertionSortList(ListNode* head) {
        ListNode* dummy = new ListNode(0, head);

        // find out how much iteration
        int length = 0;
        while (head != nullptr) {
            head = head->next;
            length++;
        }

        // do the sorting
        head = dummy;
        for (int i = 0; i < length; i++) {
            ListNode* before = head;
            ListNode* current = head->next;
            ListNode* smallest = current;
            ListNode* beforeSmall = before;

            while (current != nullptr) {
                if (current->val < smallest->val) {
                    smallest = current;
                    beforeSmall = before;
                }

                current = current->next;
                before = before->next;
            }

            beforeSmall->next = smallest->next;
            smallest->next = head->next;
            head->next = smallest;
            head = head->next;
        }
        
        // return the result
        return dummy->next;
    }
};
```

