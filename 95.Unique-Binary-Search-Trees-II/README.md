# 95. Unique Binary Search Trees II

### Question Description

Given an integer n, return all the structurally unique BST's (binary search trees), which has exactly n nodes of unique values from 1 to n. Return the answer in any order.

```
Input: n = 3
Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
```

```
Input: n = 1
Output: [[1]]
```

### Solution

对于每一个值i，考虑[0,i-1]构成的全部左子树与[i+1,n]构成的全部右子树的所有可能组合。

### Code implemption

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
    vector<TreeNode*> generateTrees(int n) {
        if (n == 0) return {};
        return generateTreesHelper(1, n);
    }

private:
    vector<TreeNode*> generateTreesHelper(int start, int end) {
        vector<TreeNode*> allTrees;
        // Base case: if start > end, return a vector with nullptr
        if (start > end) {
            allTrees.push_back(nullptr);
            return allTrees;
        }

        // Try every number as the root node
        for (int i = start; i <= end; ++i) {
            // Generate all possible left subtrees with values less than `i`
            vector<TreeNode*> leftTrees = generateTreesHelper(start, i - 1);
            // Generate all possible right subtrees with values greater than `i`
            vector<TreeNode*> rightTrees = generateTreesHelper(i + 1, end);

            // Combine each left and right subtree with the current root node `i`
            for (TreeNode* left : leftTrees) {
                for (TreeNode* right : rightTrees) {
                    TreeNode* currentTree = new TreeNode(i);
                    currentTree->left = left;
                    currentTree->right = right;
                    allTrees.push_back(currentTree);
                }
            }
        }
        return allTrees;
    }
};
```