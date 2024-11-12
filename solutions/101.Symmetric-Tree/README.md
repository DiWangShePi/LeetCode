# 101. Symmetric Tree

### Description

Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).

**Example: **

```
Input: root = [1,2,2,3,4,4,3]
Output: true
```

```
Input: root = [1,2,2,null,3,null,3]
Output: false
```

### Solution

对称走位

### Implementation

###### c++

```
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
    bool isSymmetric(TreeNode* root) {
        if (root->left == nullptr && root->right == nullptr) return true;
        if (root->left == nullptr || root->right == nullptr) return false;

        return check(root->left, root->right); 
    }

    bool check(TreeNode* left, TreeNode* right) {
        if (left == nullptr && right == nullptr) return true;
        if (left == nullptr || right == nullptr) return false; 

        if (left->val != right->val) return false;
        return check(left->left, right->right) && check(left->right, right->left);
    }
};
```
