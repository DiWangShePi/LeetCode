# 199. Binary Tree Right Side View

### Description

Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.

### Solution

广度优先遍历，每遍历一层将该层最后一个元素放入到答案列表中。

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
    vector<int> rightSideView(TreeNode* root) {
        if (root == nullptr) return {};

        vector<TreeNode*> queue;
        vector<int> answer;
        queue.push_back(root);

        int lastLength = 0;
        int length = 1;
        while (lastLength != length) {
            answer.push_back(queue[length - 1]->val);

            for (int i = lastLength; i < length; i++) {
                TreeNode* current = queue[i];
                if (current->left != nullptr) queue.push_back(current->left);
                if (current->right != nullptr) queue.push_back(current->right);
            }
            lastLength = length;
            length = queue.size();
        }
        return answer;
    }
};
```
