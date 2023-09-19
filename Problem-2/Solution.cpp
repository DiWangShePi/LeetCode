using namespace std;

#include<iostream>
#include<list>
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

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        return add(l1, l2, 0);
    }

    ListNode* add(ListNode* node1, ListNode* node2, int addNum = 0) {
        if (!node1 && !node2 && addNum == 0){
            return nullptr;
        }

        int val1 = (node1) ? node1->val : 0;
        int val2 = (node2) ? node2->val : 0;

        int sum = val1 + val2 + addNum;

        ListNode* currentNode = new ListNode(sum % 10);
        if (node1 || node2) {
            currentNode->next = add((node1) ? node1->next:nullptr, (node2) ? node2->next:nullptr, sum/10);
        }
        return currentNode;
    }
};
