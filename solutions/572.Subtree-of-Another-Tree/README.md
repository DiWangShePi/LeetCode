# 572. Subtree of Another Tree

**Tags:** DFS

### Description

Given the roots of two binary trees root and subRoot, return true if there is a subtree of root with the same structure and node values of subRoot and false otherwise.

A subtree of a binary tree tree is a tree that consists of a node in tree and all of this node's descendants. The tree tree could also be considered as a subtree of itself.

### Example

###### Example I

![](./subtree1-tree.jpg)

> Input: root = [3,4,5,1,2], subRoot = [4,1,2]
> Output: true

###### Example II

![](./subtree2-tree.jpg)

> Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
> Output: false

### Solution

深度优先搜索和暴力枚举

> 不然凭什么叫简单题

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
    bool isSubtree(TreeNode* root, TreeNode* subRoot) {
        return dfs(root, subRoot);
    }

private:
    bool check(TreeNode* o, TreeNode* t) {
        if (!o && !t) return true;
        if ((!o && t) || (o && !t) || (o->val != t->val)) return false;
        return check(o->left, t->left) && check(o->right, t->right);
    }

    bool dfs(TreeNode* o, TreeNode* t) {
        if (!o) return false;
        return check(o, t) || dfs(o->left, t) || dfs(o->right, t);
    }
};
```
