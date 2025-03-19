# 230. Kth Smallest Element in a BST

### Description

Given the root of a binary search tree, and an integer k, return the kth smallest value (1-indexed) of all the values of the nodes in the tree.

### Solution

暴力的方式是遍历一遍，BST经中序遍历得到的数据是有序的，返回指定数值即可。



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
    int kthSmallest(TreeNode* root, int k) {
        vector<int> result;
        dfs(root, result);
        return result[k-1];
    }

private:
    void dfs(TreeNode* current, vector<int>& result) {
        if (current == nullptr) return;

        dfs(current->left, result);
        result.push_back(current->val);
        dfs(current->right, result);
    }
};
```
