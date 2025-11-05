# 559. Maximum Depth of N-ary Tree

### Description

Given a n-ary tree, find its maximum depth.

The maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.

Nary-Tree input serialization is represented in their level order traversal, each group of children is separated by the null value (See examples).

### Example

###### Example I

![](./narytreeexample.png)

> Input: root = [1,null,3,2,4,null,5,6]
> Output: 3

###### Example II

![](./sample_4_964.png)

> Input: root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
> Output: 5

### Solution

广度优先遍历

```c++
/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};
*/

class Solution {
public:
    int maxDepth(Node* root) {
        if (root == nullptr) return 0;

        vector<Node*> queue{root};

        int l = 0, r = queue.size(), an = 0;
        Node* c;
        while (l < r) {
            for (int i = l; i < r; i++) {
                c = queue[i];
                for (Node* child : c->children) {
                    queue.push_back(child);
                }
            }

            l = r;
            r = queue.size();
            an++;
        }
        return an;
    }
};
```
