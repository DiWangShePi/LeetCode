# 94. Binary Tree Inorder Traversal

### Question Description

Given the root of a binary tree, return the inorder traversal of its nodes' values.

**Example: **

```
Input: root = [1,null,2,3]

Output: [1,3,2]
```

```
Input: root = [1,2,3,4,5,null,8,null,null,6,7,9]

Output: [4,2,6,5,7,1,3,9,8]
```

### Solution

按要求实现即可

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
    vector<int> inorderTraversal(TreeNode* root) {
        vector<int> result;
        dfs(root, result);
        return result;
    }

    void dfs(TreeNode* current, vector<int>& result) {
        if (current == nullptr) {
            return;
        }

        dfs(current->left, result);
        result.push_back(current->val);
        dfs(current->right, result);
    }
};
```