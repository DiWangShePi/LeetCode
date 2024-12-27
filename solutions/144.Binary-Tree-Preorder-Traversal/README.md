# 144. Binary Tree Preorder Traversal

### Description

Given the root of a binary tree, return the preorder traversal of its nodes' values.

### Solution

按要求递归实现即可

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
    vector<int> preorderTraversal(TreeNode* root) {
        vector<int> result;
        traversal(root, result);
        return result;
    }

    void traversal(TreeNode* current, vector<int>& result){
        if (!current) return;

        result.push_back(current->val);
        traversal(current->left, result);
        traversal(current->right, result);
    }
};
```
