# 110. Balanced Binary Tree

### Description

Given a binary tree, determine if it is height-balanced.

**Example: **

```
Input: root = [3,9,20,null,null,15,7]
Output: true
```

```
Input: root = [1,2,2,3,3,null,null,4,4]
Output: false
```

```
Input: root = []
Output: true
```

### Solution

定义递归方法，查询左子树和右子树的高度。若两者相差小于2，则为true，否则为false。

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
    bool isBalanced(TreeNode* root) {
        if (root == nullptr) return true;

        int leftHeight = treeHeight(root->left) + 1;
        int rightHeight = treeHeight(root->right) + 1;
        if (leftHeight == 0 || rightHeight == 0) return false;
        return abs(leftHeight - rightHeight) < 2;
    }

    int treeHeight(TreeNode* current) {
        if (current == nullptr) return 0;

        int leftHeight = treeHeight(current->left) + 1;
        int rightHeight = treeHeight(current->right) + 1;

        if (leftHeight == 0 || rightHeight == 0) return -1;
        if (abs(leftHeight - rightHeight) > 1) return -1;
        return max(leftHeight, rightHeight);
    }
};
```