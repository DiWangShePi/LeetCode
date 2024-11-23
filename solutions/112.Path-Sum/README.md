# 112. Path Sum

### Description

Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.

A leaf is a node with no children.

### Solution

树搜索，在每一个叶子节点中检查这一路加下来的结果是否为target

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
    bool hasPathSum(TreeNode* root, int targetSum) {
        if (root == nullptr) return false;
        if (root->left == nullptr && root->right == nullptr) return root->val == targetSum;
        return pathSum(root->left, targetSum, root->val) || pathSum(root->right, targetSum, root->val);
    }

    bool pathSum(TreeNode* current, int targetSum, int value) {
        if (current == nullptr) return false;
        
        value = value + current->val;
        if (current->left == nullptr && current->right == nullptr) return targetSum == value;
        return pathSum(current->left, targetSum, value) || pathSum(current->right, targetSum, value);
    }
};
```