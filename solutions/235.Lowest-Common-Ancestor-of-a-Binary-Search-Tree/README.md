# 235. Lowest Common Ancestor of a Binary Search Tree

### Description

Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes in the BST.

According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).”

### Solution

一个直观的方式是两次遍历，找到两个从根到目标节点的路径，然后在路径中寻找第一个分叉的节点，即为目标节点。

但为什么要拿到完整路径才能进行判断呢？我们在遍历的过程中其实就可以判断了。如果两个目标节点对于当前节点来说，“意见”不一，则当前节点即为目标节点。

### Implementation

###### c++

```c++
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */

class Solution {
public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        TreeNode* target = root;
        while (true) {
            if (p->val > target->val && q->val > target->val) target = target->right;
            else if (p->val < target->val && q->val < target->val) target = target->left;
            else break;
        }
        return target;
    }
};
```
