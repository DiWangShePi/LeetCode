# 404. Sum of Left Leaves

### Description

Given the root of a binary tree, return the sum of all left leaves.

A leaf is a node with no children. A left leaf is a leaf that is the left child of another node.

### Example 

###### Example I

![](./leftsum-tree.jpg)

> Input: root = [3,9,20,null,null,15,7]
> Output: 24
> Explanation: There are two left leaves in the binary tree, with values 9 and 15 respectively.

###### Example II

> Input: root = [1]
> Output: 0

### Solution

遍历一遍树，将所有的左节点放入队列中，再逐个取出，检查其是否符合child node的定义。将符合的元素值求和。

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
    int sumOfLeftLeaves(TreeNode* root) {
        vector<TreeNode*> leaf;
        helper(root, leaf);

        int an = 0;
        for (TreeNode* node : leaf) {
            if (node->left == nullptr && node->right == nullptr)
                an += node->val;
        }
        return an;
    }

private:
    void helper(TreeNode* current, vector<TreeNode*>& leaf) {
        if (current == nullptr) return;

        if (current->left != nullptr)
            leaf.push_back(current->left);

        helper(current->left, leaf);
        helper(current->right, leaf);
    }
};
```

当然，我们也可以在遍历的过程中检查节点是否符合要求，或者在遍历到叶子节点时，提前告诉当前节点，你是左子节点还是右子节点。

