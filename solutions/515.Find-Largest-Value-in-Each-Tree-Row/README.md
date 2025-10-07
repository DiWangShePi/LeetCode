# 515. Find Largest Value in Each Tree Row

**Tags:** BFS

### Description

Given the root of a binary tree, return an array of the largest value in each row of the tree (0-indexed).

### Example 

###### Example I

![](./largest_e1.jpg)

> Input: root = [1,3,2,5,3,null,9]
> Output: [1,3,9]

###### Example II

> Input: root = [1,2,3]
> Output: [1,3]

### Solution

广度优先搜索

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
    vector<int> largestValues(TreeNode* root) {
        if (root == nullptr) return {};

        vector<TreeNode*> queue{root};
        vector<int> ans;
        int l = 0, r = 1;
        while (l != r) {
            int an = INT_MIN;
            for (int i = l; i < r; i++) {
                TreeNode* c = queue[i];
                an = max(an, c->val);

                if (c->left != nullptr) queue.push_back(c->left);
                if (c->right != nullptr) queue.push_back(c->right);
            }

            l = r;
            r = queue.size();
            ans.push_back(an);
        }
        return ans;
    }
};
```
