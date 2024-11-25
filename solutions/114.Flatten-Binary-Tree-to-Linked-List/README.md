# 114. Flatten Binary Tree to Linked List

### Description

Given the root of a binary tree, flatten the tree into a "linked list":

- The "linked list" should use the same TreeNode class where the right child pointer points to the next node in the list and the left child pointer is always null.
- The "linked list" should be in the same order as a pre-order traversal of the binary tree.

### Solution

若当前节点存在左子树，则找到左子树的最右节点，将当前节点的右节点指向该最右节点，当前节点指向该节点。

重复此过程，直到所有节点均不存在左子树。

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
    void flatten(TreeNode* root) {
        if (root == nullptr) return;

        if (root->left != nullptr) {
            TreeNode* subLeft = root->left;
            while (subLeft->right != nullptr) subLeft = subLeft->right;
            subLeft->right = root->right;
            root->right = root->left;
            root->left = nullptr;
        }
        flatten(root->right);
    }
};
```