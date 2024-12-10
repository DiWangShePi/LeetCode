# 129. Sum Root to Leaf Numbers

### Description

You are given the root of a binary tree containing digits from 0 to 9 only.

Each root-to-leaf path in the tree represents a number.

For example, the root-to-leaf path 1 -> 2 -> 3 represents the number 123.
Return the total sum of all root-to-leaf numbers. Test cases are generated so that the answer will fit in a 32-bit integer.

A leaf node is a node with no children.

### Solution

深度优先搜索，每次搜到底层就将当前表示的数字加上去

### Implementation

###### c++

```c++
class Solution {
public:
    int sumNumbers(TreeNode* root) {
        return sum(root, 0);
    }

    int sum(TreeNode* node, int currentSum) {
        if (node == nullptr) return 0;

        currentSum = currentSum * 10 + node->val;
        if (node->left == nullptr && node->right == nullptr) {
            return currentSum;
        }
        return sum(node->left, currentSum) + sum(node->right, currentSum);
    }
};
```