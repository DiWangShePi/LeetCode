# 103. Binary Tree Zigzag Level Order Traversal

### Description

Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).

**Example: **

```
Input: root = [3,9,20,null,null,15,7]
Output: [[3],[20,9],[15,7]]
```

```
Input: root = [1]
Output: [[1]]
```

```
Input: root = []
Output: []
```

### Solution

与上一题一样，根据深入的层数分插入在前面还是在后面

### Implemptation

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
    vector<vector<int>> zigzagLevelOrder(TreeNode* root) {
            vector<vector<int>> result;
            if (!root) return result;

            queue<TreeNode*> q;
            q.push(root);
            bool leftToRight = true;

            while (!q.empty()) {
                int levelSize = q.size();
                deque<int> levelNodes;  
                for (int i = 0; i < levelSize; ++i) {
                    TreeNode* node = q.front();
                    q.pop();

                    if (leftToRight) {
                        levelNodes.push_back(node->val); 
                    } else {
                        levelNodes.push_front(node->val); 
                    }

                    if (node->left) q.push(node->left);  
                    if (node->right) q.push(node->right); 
                }

                result.push_back(vector<int>(levelNodes.begin(), levelNodes.end()));
                leftToRight = !leftToRight; 
            }

            return result;
    }
};
```