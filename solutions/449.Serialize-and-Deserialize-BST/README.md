# 449. Serialize and Deserialize BST

**Tags:** BST

### Description

Serialization is converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.

Design an algorithm to serialize and deserialize a binary search tree. There is no restriction on how your serialization/deserialization algorithm should work. You need to ensure that a binary search tree can be serialized to a string, and this string can be deserialized to the original tree structure.

The encoded string should be as compact as possible.

### Example

###### Example I

> Input: root = [2,1,3]
> Output: [2,1,3]

###### Example II

> Input: root = []
> Output: []

### Solution

后序遍历，每个节点值之间用空格隔开，利用二叉树性质确认分支是否到达底部，不需要用'#'来显式指示。

```c++
class Codec {
public:

    // Encodes a tree to a single string.
    string serialize(TreeNode* root) {
        stringstream ss;
        serializeHelper(root, ss);
        return ss.str();
    }

    // Decodes your encoded data to tree.
    TreeNode* deserialize(string data) {
        if (data.empty()) return nullptr;
        stringstream ss(data);
        vector<int> values;
        string token;
        while (getline(ss, token, ' ')) {
            values.push_back(stoi(token));
        }
        int index = 0;
        return deserializeHelper(values, index, INT_MIN, INT_MAX);
    }

private:
    void serializeHelper(TreeNode* node, stringstream& ss) {
        if (!node) return;
        ss << node->val << " ";
        serializeHelper(node->left, ss);
        serializeHelper(node->right, ss);
    }

    TreeNode* deserializeHelper(const vector<int>& values, int& index, int lower, int upper) {
        if (index >= values.size()) return nullptr;
        int val = values[index];
        if (val < lower || val > upper) return nullptr;
        
        TreeNode* node = new TreeNode(val);
        index++;
        node->left = deserializeHelper(values, index, lower, val);
        node->right = deserializeHelper(values, index, val, upper);
        return node;
    }
};

// Your Codec object will be instantiated and called as such:
// Codec* ser = new Codec();
// Codec* deser = new Codec();
// string tree = ser->serialize(root);
// TreeNode* ans = deser->deserialize(tree);
// return ans;
```
