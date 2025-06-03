# 337. House Robber III

### Description

The thief has found himself a new place for his thievery again. There is only one entrance to this area, called root.

Besides the root, each house has one and only one parent house. After a tour, the smart thief realized that all houses in this place form a binary tree. It will automatically contact the police if two directly-linked houses were broken into on the same night.

Given the root of the binary tree, return the maximum amount of money the thief can rob without alerting the police.

### Example 

###### Example I

![](./rob1-tree.jpg)

```
Input: root = [3,2,3,null,3,null,1]
Output: 7
Explanation: Maximum amount of money the thief can rob = 3 + 3 + 1 = 7.
```

###### Example II

![](./rob2-tree.jpg)

```
Input: root = [3,4,5,1,3,null,1]
Output: 9
Explanation: Maximum amount of money the thief can rob = 4 + 5 = 9.
```

### Solution

用树形动态规划，通过后序遍历递归地计算每个节点在“偷”与“不偷”两种情况下的最大收益。如果偷当前节点，则不能偷它的左右子节点；如果不偷当前节点，则可以选择偷或不偷其子节点以获得更大利润。每个节点返回一个二元组，分别表示“不偷当前节点”和“偷当前节点”的最大收益，最终结果为根节点这两种收益的较大值。

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
    int rob(TreeNode* root) {
        pair<int, int> result = robber(root);
        return max(result.first, result.second);
    }

private:
    pair<int, int> robber(TreeNode* current) {
        if (current == nullptr) return {0, 0};

        pair<int, int> l = robber(current->left);
        pair<int, int> r = robber(current->right);

        int do_rob = current->val + l.first + r.first;
        int do_not_rob = max(l.first, l.second) + max(r.first, r.second);
        return {do_not_rob, do_rob};
    }
};
```
