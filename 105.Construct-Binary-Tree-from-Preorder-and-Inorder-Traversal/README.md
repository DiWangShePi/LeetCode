# 105. Construct Binary Tree from Preorder and Inorder Traversal

### Description

Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.

**Example: **

```
Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
Output: [3,9,20,null,null,15,7]
```

```
Input: preorder = [-1], inorder = [-1]
Output: [-1]
```

### Solution

前序遍历的第一个节点即为根节点，在中序遍历中查找该节点，左边的即为左子树，右边的即为右子树。
进而，我们得以判断前序遍历中，接下来左子树的长度和右子树的长度。分割后进入下一层。

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
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        if (preorder.empty() || inorder.empty()) return nullptr;

        int currentValue = preorder[0];

        int index = 0;
        for (; index < inorder.size(); index++) {
            if (inorder[index] == currentValue) break;
        }

        vector<int> inLeft(inorder.begin(), inorder.begin() + index);
        vector<int> inRight(inorder.begin() + index + 1, inorder.end());

        vector<int> preLeft(preorder.begin() + 1, preorder.begin() + 1 + index);
        vector<int> preRight(preorder.begin() + 1 + index, preorder.end());

        TreeNode* current = new TreeNode(currentValue);
        current->left = buildTree(preLeft, inLeft);
        current->right = buildTree(preRight, inRight);

        return current;
    }
};
```

a much faster solution:

```c++
class Solution {
public:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        return buildTree(preorder.begin(), preorder.end(), inorder.begin(), inorder.end());
    }

private:
    using Iter = vector<int>::iterator;

    TreeNode* buildTree(Iter preBegin, Iter preEnd, Iter inBegin, Iter inEnd) {
        if (preBegin == preEnd || inBegin == inEnd) return nullptr;

        int currentValue = *preBegin;
        Iter inRoot = find(inBegin, inEnd, currentValue);
        auto leftSize = distance(inBegin, inRoot);

        TreeNode* current = new TreeNode(currentValue);
        current->left = buildTree(preBegin + 1, preBegin + 1 + leftSize, inBegin, inRoot);
        current->right = buildTree(preBegin + 1 + leftSize, preEnd, inRoot + 1, inEnd);

        return current;
    }
};
```