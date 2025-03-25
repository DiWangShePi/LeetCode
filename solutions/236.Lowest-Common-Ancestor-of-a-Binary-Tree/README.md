# 236. Lowest Common Ancestor of a Binary Tree

### Description

Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.

According to the definition of LCA on Wikipedia: "The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).""

### Solution

笨办法，遍历两遍树，找到从根到目标节点的路径，再比对路径。

好一点的办法：递归。
- 如果当前节点为空，或者等于p或者q，则返回当前节点。
- 在左子树和右子树中分别搜索。如果两个均不为空，即代表当前节点为目标。若一个为空而另一个不为空，则返回不为空的哪一个。

### Implementation

###### c++

```c++
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        vector<TreeNode*> path1, path2;
        bool found1 = false;
        bool found2 = false;
        search(p, root, path1, found1);
        search(q, root, path2, found2);

        int length = min(path1.size(), path2.size());
        for (int i = 1; i < length; i++) {
            if (path1[i] != path2[i]) return path1[i - 1];
        }
        return path1[length - 1];
    }

private:
    void search(TreeNode* target, TreeNode* current, vector<TreeNode*>& path, bool& found) {
        if (current == nullptr || found) return;

        path.push_back(current);
        if (current == target) {
            found = true;
            return;
        }
        search(target, current->left, path, found);
        search(target, current->right, path, found);

        if (!found) path.pop_back();
    }
};
```

```c++
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        if (root == nullptr || root == p || root == q) return root;

        TreeNode* left = lowestCommonAncestor(root->left, p, q);
        TreeNode* right = lowestCommonAncestor(root->right, p, q);

        if (left != nullptr && right != nullptr) return root;
        return left != nullptr ? left : right;
    }
};
```