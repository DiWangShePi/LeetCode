# 111. Minimum Depth of Binary Tree

### Description

Given a binary tree, find its minimum depth.

The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.

Note: A leaf is a node with no children.

### Solution

递归，每个节点的最小深度是左和右中较小的哪一个。

### Implementation

###### c++

```c++
#include <iostream>
#include <queue>
#include <algorithm>
using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

class Solution {
public:
    int minDepth(TreeNode* root) {
        if (root == nullptr) return 0;

        int leftHeight = minDepth(root->left);
        int rightHeight = minDepth(root->right);

        if (leftHeight == 0 || rightHeight == 0) {
            return leftHeight + rightHeight + 1;
        }

        return min(leftHeight, rightHeight) + 1;
    }
};

TreeNode* createNode(int val) {
    return new TreeNode(val);
}

void freeTree(TreeNode* root) {
    if (!root) return;
    freeTree(root->left);
    freeTree(root->right);
    delete root;
}

void testCase(TreeNode* root, int expected) {
    Solution solution;
    int result = solution.minDepth(root);
    cout << "Min Depth: " << result << " | Expected: " << expected << " | "
         << (result == expected ? "PASS" : "FAIL") << endl;
}

int main() {
    // 测试案例 1：空树
    testCase(nullptr, 0);

    // 测试案例 2：只有一个节点
    TreeNode* root1 = createNode(1);
    testCase(root1, 1);
    freeTree(root1);

    // 测试案例 3：完全二叉树
    TreeNode* root2 = createNode(1);
    root2->left = createNode(2);
    root2->right = createNode(3);
    root2->left->left = createNode(4);
    root2->left->right = createNode(5);
    root2->right->left = createNode(6);
    root2->right->right = createNode(7);
    testCase(root2, 2);
    freeTree(root2);

    // 测试案例 4：只有左子树
    TreeNode* root3 = createNode(1);
    root3->left = createNode(2);
    root3->left->left = createNode(3);
    root3->left->left->left = createNode(4);
    testCase(root3, 4);
    freeTree(root3);

    // 测试案例 5：只有右子树
    TreeNode* root4 = createNode(1);
    root4->right = createNode(2);
    root4->right->right = createNode(3);
    root4->right->right->right = createNode(4);
    testCase(root4, 4);
    freeTree(root4);

    // 测试案例 6：非对称树
    TreeNode* root5 = createNode(1);
    root5->left = createNode(2);
    root5->right = createNode(3);
    root5->right->right = createNode(4);
    root5->right->right->right = createNode(5);
    testCase(root5, 2);
    freeTree(root5);

    return 0;
}
```