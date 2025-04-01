# 257. Binary Tree Paths

### Description

Given the root of a binary tree, return all root-to-leaf paths in any order.

A leaf is a node with no children.

### Solution

遍历树

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
    vector<string> binaryTreePaths(TreeNode* root) {
        vector<string> result;
        if (root) {
            string path;
            search(root, path, result);
        }
        return result;
    }

private:
    void search(TreeNode* node, string& path, vector<string>& result) {
        size_t old_size = path.size();
        if (!path.empty()) path += "->";
        path += to_string(node->val);

        if (!node->left && !node->right) {
            result.push_back(path);
        } else {
            if (node->left) search(node->left, path, result);
            if (node->right) search(node->right, path, result);
        }

        path.resize(old_size);
    }
};
```
