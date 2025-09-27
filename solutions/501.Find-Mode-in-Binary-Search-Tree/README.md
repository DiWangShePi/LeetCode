# 501. Find Mode in Binary Search Tree

### Description

Given the root of a binary search tree (BST) with duplicates, return all the mode(s) (i.e., the most frequently occurred element) in it.

If the tree has more than one mode, return them in any order.

Assume a BST is defined as follows:

- The left subtree of a node contains only nodes with keys less than or equal to the node's key.
- The right subtree of a node contains only nodes with keys greater than or equal to the node's key.
- Both the left and right subtrees must also be binary search trees.

### Example 

###### Example I

![](./mode-tree.jpg)

> Input: root = [1,null,2,2]
> Output: [2]

###### Example II

> Input: root = [0]
> Output: [0]

### Solution

暴力解法：遍历整个树，记录所有数字出现的频率，选出最高的。

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
    vector<int> findMode(TreeNode* root) {
        unordered_map<int, int> dict;
        dfs(dict, root);

        int t = INT_MIN;
        vector<int> an;
        for (auto it = dict.begin(); it != dict.end(); it++) {
            if (it->second > t) {
                t = it->second;

                an.clear();
                an.push_back(it->first);
            } else if (it->second == t) an.push_back(it->first);
        }
        return an;
    }

private:
    void dfs(unordered_map<int, int>& dict, TreeNode* current) {
        if (current == nullptr) return;

        if (dict.count(current->val) != 0) dict[current->val]++;
        else dict[current->val] = 1;

        dfs(dict, current->left);
        dfs(dict, current->right);
    }
};
```

我们也可以使用 Morris 中序遍历来只用O（1）的额外空间

```c++
class Solution {
public:
    int base, count, maxCount;
    vector<int> answer;

    void update(int x) {
        if (x == base) {
            ++count;
        } else {
            count = 1;
            base = x;
        }
        if (count == maxCount) {
            answer.push_back(base);
        }
        if (count > maxCount) {
            maxCount = count;
            answer = vector<int> {base};
        }
    }

    vector<int> findMode(TreeNode* root) {
        TreeNode *cur = root, *pre = nullptr;
        while (cur) {
            if (!cur->left) {
                update(cur->val);
                cur = cur->right;
                continue;
            }
            pre = cur->left;
            while (pre->right && pre->right != cur) {
                pre = pre->right;
            }
            if (!pre->right) {
                pre->right = cur;
                cur = cur->left;
            } else {
                pre->right = nullptr;
                update(cur->val);
                cur = cur->right;
            }
        }
        return answer;
    }
};
```
