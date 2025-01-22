# 173. Binary Search Tree Iterator

### Description

Implement the BSTIterator class that represents an iterator over the in-order traversal of a binary search tree (BST):

- STIterator(TreeNode root) Initializes an object of the BSTIterator class. The root of the BST is given as part of the constructor. The pointer should be initialized to a non-existent number smaller than any element in the BST.
- boolean hasNext() Returns true if there exists a number in the traversal to the right of the pointer, otherwise returns false.
- int next() Moves the pointer to the right, then returns the number at the pointer.
Notice that by initializing the pointer to a non-existent smallest number, the first call to next() will return the smallest element in the BST.

You may assume that next() calls will always be valid. That is, there will be at least a next number in the in-order traversal when next() is called.

### Solution

按要求实现即可

### Implementation

###### c++

> gpt claims this to be more efficent, and it does beat mine. However, the code that beats this code looks like mine.

```c++
class BSTIterator {
    stack<TreeNode*> nodeStack;

public:
    BSTIterator(TreeNode* root) {
        pushLeft(root);
    }
    
    int next() {
        TreeNode* current = nodeStack.top();
        nodeStack.pop();
        int result = current->val;
        
        pushLeft(current->right);
        return result;
    }
    
    bool hasNext() {
        return !nodeStack.empty();
    }

private:
    void pushLeft(TreeNode* node) {
        while (node != nullptr) {
            nodeStack.push(node);
            node = node->left;
        }
    }
};
```

> Original version

```c++
class BSTIterator {
    int pointer = 0;
    vector<int> nodeList;

public:
    BSTIterator(TreeNode* root) {
        inorder(root, nodeList);
    }
    
    int next() {
        return nodeList[pointer++];
    }
    
    bool hasNext() {
        return pointer < nodeList.size();
    }

private:
     void inorder(TreeNode* current, vector<int>& nodeList) {
        if (current == nullptr) return;

        inorder(current->left, nodeList);
        nodeList.push_back(current->val);
        inorder(current->right, nodeList);
    }
};
```