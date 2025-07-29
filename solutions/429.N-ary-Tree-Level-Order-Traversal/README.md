# 429. N-ary Tree Level Order Traversal

### Description

Given an n-ary tree, return the level order traversal of its nodes' values.

Nary-Tree input serialization is represented in their level order traversal, each group of children is separated by the null value (See examples).

### Example

###### Example I

![](./narytreeexample.png)

> Input: root = [1,null,3,2,4,null,5,6]
> Output: [[1],[3,2,4],[5,6]]

###### Example II

![](./sample_4_964.png)

> Input: root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
> Output: [[1],[2,3,4,5],[6,7,8,9,10],[11,12,13],[14]]

### Solution

广度优先搜索

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
    vector<vector<int>> levelOrder(Node* root) {
        if (root == nullptr) return {};

        vector<vector<int>> an;
        vector<Node*> queue{root};
        int l = 0, r = 1;

        vector<int> current;
        while (l < r) {
            for (int i = l; i < r; i++) {
                Node* c = queue[i];

                current.push_back(c->val);
                for (Node* n : c->children) {
                    queue.push_back(n);
                }
            }

            l = r;
            r = queue.size();
            an.push_back(current);
            current.clear();
        }
        return an;
    }
};
```
