# 124. Binary Tree Maximum Path Sum

### Description

A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence at most once. Note that the path does not need to pass through the root.

The path sum of a path is the sum of the node's values in the path.

Given the root of a binary tree, return the maximum path sum of any non-empty path.

### Solution

递归更新

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
    int maxPathSum(TreeNode* root) {
        int maxPath = INT_MIN;
        getMaxPath(root, maxPath);
        return maxPath;
    }

    int getMaxPath(TreeNode* current, int& maxPath) {
        if (current == nullptr) return 0;

        int gainOnLeft = max(getMaxPath(current->left, maxPath), 0);
        int gainOnRight = max(getMaxPath(current->right, maxPath), 0);

        int currentMax = current->val + gainOnLeft + gainOnRight;
        maxPath = max(maxPath, currentMax);
        return current->val + max(gainOnLeft, gainOnRight);
    }
};
```