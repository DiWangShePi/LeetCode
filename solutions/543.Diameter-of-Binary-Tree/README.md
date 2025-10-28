# 543. Diameter of Binary Tree

### Description

Given the root of a binary tree, return the length of the diameter of the tree.

The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.

The length of a path between two nodes is represented by the number of edges between them.

### Example 

###### Example I

![](./diamtree.jpg)

> Input: root = [1,2,3,4,5]
> Output: 3
> Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].

###### Example II

> Input: root = [1,2]
> Output: 1

### Solution

遍历树，计算每个节点左子树的高度和右子树的高度，以此更新当前节点所可能存在的最长直径。

因为只是二叉树，不是二叉搜索树，所以这个树不是平衡的，直径不一定经过根节点。

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
    int diameterOfBinaryTree(TreeNode* root) {
        int diameter = 0;
        height(root, diameter);
        return diameter;
    }

private:
    int height(TreeNode* node, int& diameter) {
        if (node == nullptr) return 0;
        
        int leftHeight = height(node->left, diameter);
        int rightHeight = height(node->right, diameter);
        
        diameter = max(diameter, leftHeight + rightHeight);
        
        return 1 + max(leftHeight, rightHeight);
    }
};
```
