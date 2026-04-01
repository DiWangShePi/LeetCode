# 653. Two Sum IV - Input is a BST

**Tags:** Tree, Hash Map

### Description

Given the root of a binary search tree and an integer k, return true if there exist two elements in the BST such that their sum is equal to k, or false otherwise.

### Example

###### Example I

![](./sum_tree_1.jpg)

> Input: root = [5,3,6,2,4,null,7], k = 9
> Output: true

###### Example II

![](./sum_tree_2.jpg)

> Input: root = [5,3,6,2,4,null,7], k = 28
> Output: false

### Solution

跟两数之和一样，但是遍历树的过程中检查。

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
    unordered_map<int, int> dict;

public:
    bool findTarget(TreeNode* root, int k) {
        return dfs(root, k);
    }

private:
    bool dfs(TreeNode* current, int k) {
        if (current == nullptr) return false;

        if (dict.count(k - current->val) != 0) return true;
        dict[current->val] = 1;
        
        bool l = dfs(current->left, k);
        bool r = dfs(current->right, k);
        return l || r;
    }
};
```
