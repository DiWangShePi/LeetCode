# 450. Delete Node in a BST

### Description

Given a root node reference of a BST and a key, delete the node with the given key in the BST. Return the root node reference (possibly updated) of the BST.

Basically, the deletion can be divided into two stages:

1. Search for a node to remove.
2. If the node is found, delete the node.

### Example

###### Example I

![](./del_node_1.jpg)

> Input: root = [5,3,6,2,4,null,7], key = 3
> Output: [5,4,6,2,null,null,7]
> Explanation: Given key to delete is 3. So we find the node with value 3 and delete it.
> One valid answer is [5,4,6,2,null,null,7], shown in the above BST.
> Please notice that another valid answer is [5,2,6,null,4,null,7] and it's also accepted.

![](./del_node_supp.jpg)

###### Example II

> Input: root = [5,3,6,2,4,null,7], key = 0
> Output: [5,3,6,2,4,null,7]
> Explanation: The tree does not contain a node with value = 0.

###### Example III

> Input: root = [], key = 0
> Output: []

### Solution

二叉搜索树的删除可以分为以下四种情况：

1. 目标节点没有左子树，也没有右子树。直接删除
2. 目标节点有左子树，没有右子树。删除当前节点，返回左子树。
3. 目标节点没有左子树，有右子树。删除当前节点，返回右子树。
4. 目标节点有左子树，也有右子树。在右子树中找到最左边的节点（或者在左子树中找到最右边的节点），将值与当前目标节点的值交换，随后继续。

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
    TreeNode* deleteNode(TreeNode* root, int key) {
        if (root == nullptr) return nullptr;

        if (root->val == key) {
            if (root->left == nullptr && root->right == nullptr) return nullptr;
            else if (root->left && root->right == nullptr) return root->left;
            else if (root->left == nullptr && root->right) return root->right;
            else {
                TreeNode* c = find(root->right);
                int t = root->val;
                root->val = c->val;
                c->val = t;
            }
        }

        root->right = deleteNode(root->right, key);
        root->left = deleteNode(root->left, key);
        return root;
    }

private:
    TreeNode* find(TreeNode* current) {
        if (current->left != nullptr) return find(current->left);
        else return current;
    }
};
```

上述代码或许在理解上比较简明，但实现上可以再优化：

```c++
class Solution {
public:
    TreeNode* deleteNode(TreeNode* root, int key) {
        if (!root) return nullptr;

        if (key < root->val) {
            root->left = deleteNode(root->left, key);
        } else if (key > root->val) {
            root->right = deleteNode(root->right, key);
        } else {
            // 找到要删除的节点
            if (!root->left) {
                TreeNode* r = root->right;
                delete root;
                return r;
            } else if (!root->right) {
                TreeNode* l = root->left;
                delete root;
                return l;
            } else {
                // 找右子树最小节点替换
                TreeNode* minNode = findMin(root->right);
                root->val = minNode->val;
                root->right = deleteNode(root->right, minNode->val);
            }
        }
        return root;
    }

private:
    TreeNode* findMin(TreeNode* node) {
        while (node->left) node = node->left;
        return node;
    }
};
```
