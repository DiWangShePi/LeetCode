# 116. Populating Next Right Pointers in Each Node

### Description

You are given a perfect binary tree where all leaves are on the same level, and every parent has two children. The binary tree has the following definition:

> struct Node {
>   int val;
>   Node *left;
>   Node *right;
>   Node *next;
> }
Populate each next pointer to point to its next right node. If there is no next right node, the next pointer should be set to NULL.

Initially, all next pointers are set to NULL.

### Solution

广度优先遍历，在遍历每一层的时候，将当前节点的next指针指向下一个

### Implementation

###### c++

```c++
class Solution {
public:
    Node* connect(Node* root) {
        if (root == nullptr) return root;

        queue<Node*> q;
        q.push(root);

        while (!q.empty()) {
            int levelSize = q.size();
            for (int i = 0; i < levelSize; i++) {
                Node* node = q.front();
                q.pop();

                if (i < levelSize - 1) {
                    node->next = q.front();
                } else {
                    node->next = nullptr;
                }

                if (node->left) q.push(node->left);
                if (node->right) q.push(node->right);
            }
        }
        return root;
    }
};
```

Why is DFS more efficient when I use a heap? Why is DFS more efficient when I use a heap?

```c++
class Solution {
public:
    Node* connect(Node* root) {
        if (!root) return NULL;
        if (root->left) root->left->next = root->right;
        if (root->right && root->next) root->right->next = root->next->left;

        connect(root->left);
        connect(root->right);
        return root;
    }
};
```