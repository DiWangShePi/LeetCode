# 222. Count Complete Tree Nodes

### Description

Given the root of a complete binary tree, return the number of the nodes in the tree.

According to Wikipedia, every level, except possibly the last, is completely filled in a complete binary tree, and all nodes in the last level are as far left as possible. It can have between 1 and 2h nodes inclusive at the last level h.

Design an algorithm that runs in less than O(n) time complexity.

### Solution

> 其实遍历就可以了。

这是一颗完整的二叉树，因此除了最后一层以外，剩余所有层节点都是满的，上层节点的总数目可以用公式计算。

随后我们需要考虑最后一层的节点数目，题目表示最后一层所有节点都会尽可能的向左排布，这代表：
- 如果某一个位置存在节点，那么左边的所有位置都存在节点
- 如果某一个位置不存在节点，那么右边的所有位置都不存在节点
由此，我们可以使用二分查找来找到最后一个节点的位置。

### Implementation

###### c++

> 但其实还是遍历

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
    int countNodes(TreeNode* root) {
        int result = 0;
        dfs(root, result);
        return result;
    }

private:
    void dfs(TreeNode* current, int& result) {
        if (current == nullptr) return;

        result++;
        dfs(current->left, result);
        dfs(current->right, result);
    }
};
```
