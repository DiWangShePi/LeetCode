# 106. Construct Binary Tree from Inorder and Postorder Traversal

### Description

Given two integer arrays inorder and postorder where inorder is the inorder traversal of a binary tree and postorder is the postorder traversal of the same tree, construct and return the binary tree.

**Example: **

```
Input: inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
Output: [3,9,20,null,null,15,7]
```

```
Input: inorder = [-1], postorder = [-1]
Output: [-1]
```

### Solution

与上一题一样，每次取最后一个值。

### Implementation

###### c++

```c++
class Solution {
public:
    TreeNode* buildTree(vector<int>& inorder, vector<int>& postorder) {
        return buildTree(inorder.begin(), inorder.end(), postorder.begin(), postorder.end());
    }

private:
    using Iter = vector<int>::iterator;

    TreeNode* buildTree(Iter inBegin, Iter inEnd, Iter postBegin, Iter postEnd) {
        if (inBegin == inEnd || postBegin == postEnd) return nullptr;

        int currentValue = *(postEnd - 1);
        Iter inRoot = find(inBegin, inEnd, currentValue);

        auto leftSize = distance(inBegin, inRoot);
        TreeNode* current = new TreeNode(currentValue);
        current->left = buildTree(inBegin, inRoot, postBegin, postBegin + leftSize);
        current->right = buildTree(inRoot + 1, inEnd, postBegin + leftSize, postEnd - 1);

        return current;
    }
};
```

我们也可以维护一个哈希表，作为每个元素的位置（因为每个元素都是特殊的）。随后用一个传递值m表示根节点的位置。
参考如下实现：

```c++
class Solution{
public:
    TreeNode *buildTree(vector<int> &postorder, int &m, vector<int> &inorder, int i, int j,unordered_map<int,int>&mapper){
        if (i > j) return nullptr;
        if (i == j){
            m--;
            return new TreeNode(inorder[i]);
        }
        int k=mapper[postorder[m--]];
        TreeNode *right = buildTree(postorder, m, inorder, k + 1, j,mapper);
        TreeNode *left = buildTree(postorder, m, inorder, i, k - 1,mapper);
        return new TreeNode(inorder[k], left, right);
    }
    TreeNode *buildTree(vector<int>& inorder, vector<int>& postorder) {
        int m = inorder.size()-1;
        unordered_map<int,int> mapper;
        for(int i=0;i<inorder.size();i++)mapper[inorder[i]]=i;
        return buildTree(postorder, m, inorder, 0, m,mapper);
    }
};
```