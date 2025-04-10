# 98. Validate Binary Search Tree

### Question Description

Given the root of a binary tree, determine if it is a valid binary search tree (BST).

A valid BST is defined as follows:
- The left subtree of a node contains only nodes with keys less than the node's key.
- The right subtree of a node contains only nodes with keys greater than the node's key.
- Both the left and right subtrees must also be binary search trees.

**Example: **

```
Input: root = [2,1,3]
Output: true
```

```
Input: root = [5,1,4,null,null,3,6]
Output: false
Explanation: The root node's value is 5 but its right child's value is 4.
```

### Solution

递归的检查即可

### Code Implemption

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
    bool isValidBST(TreeNode* root) {
        return check(root, LLONG_MIN, LLONG_MAX);
    }
    
    bool check(TreeNode* current, long long left, long long right) {
        if (current == nullptr) return true;
        
        long long value = (long long)current->val;
        
        if (value <= left) return false;
        if (value >= right) return false;
        
        return check(current->left, left, value) && 
               check(current->right, value, right);
    }
};
```