# 513. Find Bottom Left Tree Value

### Description

Given the root of a binary tree, return the leftmost value in the last row of the tree.

### Example 

###### Example I

![](./tree1.jpg)

> Input: root = [2,1,3]
> Output: 1

###### Example II

> Input: root = [1,2,3,4,null,5,6,null,null,7]
> Output: 7

### Solution

广度优先搜索，在每一层的开始更新答案

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
    int findBottomLeftValue(TreeNode* root) {
        vector<TreeNode*> queue{root};
        int an;
        int l = 0, r = 1;

        while (l != r) {
            an = queue[l]->val;

            for (int i = l; i < r; i++) {
                TreeNode* c = queue[i];
                if (c->left != nullptr) queue.push_back(c->left);
                if (c->right != nullptr) queue.push_back(c->right);
            }

            l = r;
            r = queue.size();
        }
        return an;
    }
};
```
