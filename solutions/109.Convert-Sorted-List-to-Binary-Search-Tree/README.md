# 109. Convert Sorted List to Binary Search Tree

### Description

Given the head of a singly linked list where elements are sorted in ascending order, convert it to a 
height-balanced binary search tree.

**Example: **

```
Input: head = [-10,-3,0,5,9]
Output: [0,-3,9,-10,null,5]
Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.
```

### Solution

先转换为数组，再沿用上一题的思路即可。

### Implementation

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
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    TreeNode* sortedListToBST(ListNode* head) {
        if (head == nullptr) return nullptr;
        vector<int> nums;
        while (head != nullptr) {
            nums.push_back(head->val);
            head = head->next;
        }

        return dfs(nums, 0, nums.size() - 1);
    }

    TreeNode* dfs(vector<int> nums, int left, int right) {
        if (left > right) return nullptr;

        int mid = left + (right - left) / 2;
        TreeNode* current = new TreeNode(nums[mid]);
        current->left = dfs(nums, left, mid-1);
        current->right = dfs(nums, mid+1, right);
        return current;
    }
};
```