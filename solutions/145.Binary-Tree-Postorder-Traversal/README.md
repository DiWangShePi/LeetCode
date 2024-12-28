# 145. Binary Tree Postorder Traversal

### Description

Given the root of a binary tree, return the postorder traversal of its nodes' values.

### Solution

按要求实现即可。先加左子节点，再加右子节点，再加自己。

### Implementation

###### c++

```c++
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
    vector<int> postorderTraversal(TreeNode* root) {
        vector<int> result;
        traversal(root, result);
        return result;
    }

    void traversal(TreeNode* current, vector<int>& result) {
        if (current == nullptr) return;

        traversal(current->left, result);
        traversal(current->right, result);
        result.push_back(current->val);
        return;
    }
};
```
