# 538. Convert BST to Greater Tree

### Description

Given the root of a Binary Search Tree (BST), convert it to a Greater Tree such that every key of the original BST is changed to the original key plus the sum of all keys greater than the original key in BST.

As a reminder, a binary search tree is a tree that satisfies these constraints:

- The left subtree of a node contains only nodes with keys less than the node's key.
- The right subtree of a node contains only nodes with keys greater than the node's key.
- Both the left and right subtrees must also be binary search trees.

### Example 

###### Example I

![](./tree.png)

> Input: root = [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
> Output: [30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]

###### Example II

> Input: root = [0,null,1]
> Output: [1,null,1]

### Solution

将所有的元素提出来，逆着加一遍和

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
    TreeNode* convertBST(TreeNode* root) {
        if (root == nullptr) return root;

        vector<TreeNode*> trees;
        collect(root, trees);

        for (int i = trees.size() - 2; i > -1; i--)
            trees[i]->val += trees[i + 1]->val;
        return root;
    }

private:
    void collect(TreeNode* current, vector<TreeNode*>& trees) {
        if (current == nullptr) return;

        collect(current->left, trees);
        trees.push_back(current);
        collect(current->right, trees);
    }
};
```
