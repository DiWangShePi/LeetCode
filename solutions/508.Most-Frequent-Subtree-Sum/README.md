# 508. Most Frequent Subtree Sum

**Tags:** DFS

### Description

Given the root of a binary tree, return the most frequent subtree sum. If there is a tie, return all the values with the highest frequency in any order.

The subtree sum of a node is defined as the sum of all the node values formed by the subtree rooted at that node (including the node itself).

### Example 

###### Example I

![](./freq1-tree.jpg)

> Input: root = [5,2,-3]
> Output: [2,-3,4]

###### Example II

![](./freq2-tree.jpg)

> Input: root = [5,2,-5]
> Output: [2]

### Solution

遍历，考虑所有sum

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
    vector<int> findFrequentTreeSum(TreeNode* root) {
        unordered_map<int, int> dict;
        dfs(root, dict);

        int fre = INT_MIN;
        vector<int> an;

        for (auto it = dict.begin(); it != dict.end(); it++) {
            if (it->second > fre) {
                fre = it->second;
                an = vector<int>{it->first};
            } else if (it->second == fre) an.push_back(it->first);
        }
        return an;
    }

private:
    int dfs(TreeNode* current, unordered_map<int, int>& dict) {
        if (current == nullptr) return 0;

        int left = dfs(current->left, dict);
        int right = dfs(current->right, dict);
        int sum = current->val + left + right;

        if (dict.count(sum) != 0) dict[sum]++;
        else dict[sum] = 1;

        return sum;
    }
};
```
