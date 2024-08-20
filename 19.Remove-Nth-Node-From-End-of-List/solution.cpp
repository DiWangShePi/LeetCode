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
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode* dummy = new ListNode(0, head);
        ListNode* currentNode = dummy;
        ListNode* targetNode = dummy;

        for (int i = 0; i <= n; i++) {
            currentNode = currentNode->next;
        }

        while(currentNode != nullptr) {
            currentNode = currentNode->next;
            targetNode = targetNode->next;
        }

        ListNode* nodeToDelete = targetNode->next;
        targetNode->next = targetNode->next->next;

        ListNode* newHead = dummy->next;

        delete nodeToDelete;
        delete dummy;

        return newHead;
    }
};