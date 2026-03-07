# 589. N-ary Tree Preorder Traversal

**Tags** Tree

### Description

Given the root of an n-ary tree, return the preorder traversal of its nodes' values.

Nary-Tree input serialization is represented in their level order traversal. Each group of children is separated by the null value (See examples)

### Example

###### Example I

![](./narytreeexample.png)

> Input: root = [1,null,3,2,4,null,5,6]
> Output: [1,3,5,6,2,4]

###### Example II

![](./sample_4_964.png)

> Input: root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
> Output: [1,2,3,6,7,11,14,4,8,12,5,9,13,10]

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
    vector<int> preorder(Node* root) {
        vector<int> ans;
        dfs(ans, root);
        return ans;
    }

private:
    void dfs(vector<int>& ans, Node* current) {
        if (current == nullptr) return;

        ans.push_back(current->val);
        for (auto* c : current->children) {
            dfs(ans, c);
        }
    }
};
```
