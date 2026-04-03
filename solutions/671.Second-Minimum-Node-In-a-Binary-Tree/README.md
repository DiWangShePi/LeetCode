# 671. Second Minimum Node In a Binary Tree

**Tags:** Tree

### Description

Given a non-empty special binary tree consisting of nodes with the non-negative value, where each node in this tree has exactly two or zero sub-node. If the node has two sub-nodes, then this node's value is the smaller value among its two sub-nodes. More formally, the property root.val = min(root.left.val, root.right.val) always holds.

Given such a binary tree, you need to output the second minimum value in the set made of all the nodes' value in the whole tree.

If no such second minimum value exists, output -1 instead.

### Example

###### Example I

![](./smbt1.jpg)

> Input: root = [2,2,5,null,null,5,7]
> Output: 5
> Explanation: The smallest value is 2, the second smallest value is 5.

###### Example II

![](./smbt2.jpg)

> Input: root = [2,2,2]
> Output: -1
> Explanation: The smallest value is 2, but there isn't any second smallest value.

### Solution

根据定义，我们知道 root 值一定是树中最小的，我们要做的是遍历这个树，找到比 root 值大的最小值。

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
    int findSecondMinimumValue(TreeNode* root) {
        long ans = dfs(root, root->val);
        return ans == LONG_MAX ? -1 : ans;
    }

private:
    long dfs(TreeNode* current, int val) {
        if (current == nullptr) return LONG_MAX;
        if (current->val > val) return current->val;

        long l = dfs(current->left, val);
        long r = dfs(current->right, val);
        return l < r ? l : r;
    }
};
```
