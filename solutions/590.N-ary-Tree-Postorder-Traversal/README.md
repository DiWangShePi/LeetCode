# 590. N-ary Tree Postorder Traversal

**Tags:** Tree

### Description

Given the root of an n-ary tree, return the postorder traversal of its nodes' values.

Nary-Tree input serialization is represented in their level order traversal. Each group of children is separated by the null value (See examples)

### Example

###### Example I

![](./narytreeexample.png)

> Input: root = [1,null,3,2,4,null,5,6]
> Output: [5,6,3,2,4,1]

###### Example II

![](./sample_4_964.png)

> Input: root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
> Output: [2,6,14,11,7,3,12,8,4,13,9,10,5,1]

### Solution

按要求实现即可。

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
    vector<int> postorder(Node* root) {
        vector<int> ans;
        post(ans, root);
        return ans;
    }

private:
    void post(vector<int>& an, Node* current) {
        if (current == nullptr) return;

        for (Node* c : current->children) {
            post(an, c);
        }
        an.push_back(current->val);
        return;
    }
};
```
