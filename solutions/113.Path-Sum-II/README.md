# 113. Path Sum II

### Description

Given the root of a binary tree and an integer targetSum, return all root-to-leaf paths where the sum of the node values in the path equals targetSum. Each path should be returned as a list of the node values, not node references.

A root-to-leaf path is a path starting from the root and ending at any leaf node. A leaf is a node with no children.

### Solution

跟上一题一样，深度搜索，满足条件时加入。

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
    vector<vector<int>> pathSum(TreeNode* root, int targetSum) {
        vector<vector<int>> result;
        vector<int> currentList;
        dfs(root, targetSum, 0, currentList, result);
        return result;
    }

    void dfs(TreeNode* current, int targetSum, int value, vector<int>& currentList, vector<vector<int>>& result) {
        if (current == nullptr) return;

        value = value + current->val;
        currentList.push_back(current->val);
        if (current->left == nullptr && current->right == nullptr) {
            if (value == targetSum) {
                result.push_back(currentList);
            }
        }
        dfs(current->left, targetSum, value, currentList, result);
        dfs(current->right, targetSum, value, currentList, result);
        currentList.pop_back();
    }
};
```