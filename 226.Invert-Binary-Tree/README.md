# 226. Invert Binary Tree

### Description

Given the root of a binary tree, invert the tree, and return its root.

### Solution

深度优先遍历，对于每一个节点，将他的左子树和右子树交换。

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
    TreeNode* invertTree(TreeNode* root) {
        invert(root);
        return root;
    }

private:
    void invert(TreeNode* current) {
        if (current == nullptr) return;
        swap(current);
        
        invert(current->left);
        invert(current->right);
    }

    void swap(TreeNode* current) {
    TreeNode* temp = current->left;
        current->left = current->right;
        current->right = temp;
    }
};
```
