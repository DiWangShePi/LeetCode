# 331. Verify Preorder Serialization of a Binary Tree

### Description

One way to serialize a binary tree is to use preorder traversal. When we encounter a non-null node, we record the node's value. If it is a null node, we record using a sentinel value such as '#'.

![](./pre-tree.jpg)

For example, the above binary tree can be serialized to the string "9,3,4,#,#,1,#,#,2,#,6,#,#", where '#' represents a null node.

Given a string of comma-separated values preorder, return true if it is a correct preorder traversal serialization of a binary tree.

It is guaranteed that each comma-separated value in the string must be either an integer or a character '#' representing null pointer.

You may assume that the input format is always valid.

For example, it could never contain two consecutive commas, such as "1,,3".
Note: You are not allowed to reconstruct the tree.

### Example 

###### Example I

```
Input: preorder = "9,3,4,#,#,1,#,#,2,#,6,#,#"
Output: true
```

###### Example II

```
Input: preorder = "1,#"
Output: false
```

###### Example III

```
Input: preorder = "9,#,#,1"
Output: false
```

### Solution

先考虑直观的做法：尝试重新构建树，考虑最终树的大小是否和给的字符串列表长度一致。

```c++
class Solution {
public:
    bool isValidSerialization(string preorder) {
        vector<string> tree;
        stringstream ss(preorder);
        string part;
        while (getline(ss, part, ',')) {
            tree.push_back(part);
        }

        int length = dfs(tree, 0);
        return length == tree.size() - 1;
    }

private:
    int dfs(vector<string> tree, int index) {
        if (index > tree.size() - 1 || tree[index] == "#") return index;

        int leftCount = dfs(tree, index + 1);
        int rightCount = dfs(tree, leftCount + 1);
        return rightCount;
    }
};
```

我们也可以考虑更深入一点的做法：我们定义slot代表每一个节点占据的位置，当我们遇到一个非空节点时，slot减一再加2，当我们遇到一个空节点时，slot减一。slot在运行过程中不应该小于0

```c++
class Solution {
public:
    bool isValidSerialization(string preorder) {
        vector<string> tree;
        stringstream ss(preorder);
        string part;
        while (getline(ss, part, ',')) {
            tree.push_back(part);
        }

        int slots = 1;
        for (const string& s : tree) {
            slots--; 
            if (slots < 0) return false; 

            if (s != "#") slots += 2;
        }
        return slots == 0;
    }
};
```

> 如果不构建vector，这个解法就可以足够快了
