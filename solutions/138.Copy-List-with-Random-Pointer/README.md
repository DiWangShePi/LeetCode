# 138. Copy List with Random Pointer

### Description

A linked list of length n is given such that each node contains an additional random pointer, which could point to any node in the list, or null.

Construct a deep copy of the list. The deep copy should consist of exactly n brand new nodes, where each new node has its value set to the value of its corresponding original node. Both the next and random pointer of the new nodes should point to new nodes in the copied list such that the pointers in the original list and copied list represent the same list state. None of the pointers in the new list should point to nodes in the original list.

For example, if there are two nodes X and Y in the original list, where X.random --> Y, then for the corresponding two nodes x and y in the copied list, x.random --> y.

Return the head of the copied linked list.

The linked list is represented in the input/output as a list of n nodes. Each node is represented as a pair of [val, random_index] where:

val: an integer representing Node.val
random_index: the index of the node (range from 0 to n-1) that the random pointer points to, or null if it does not point to any node.
Your code will only be given the head of the original linked list.

### Solution

新建字典，遍历保存

### Implementation

###### c++

```c++
/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* next;
    Node* random;
    
    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};
*/

class Solution {
public:
    Node* copyRandomList(Node* head) {
        if (head == nullptr) return nullptr;

        std::unordered_map<Node*, Node*> dict;
        Node* current = head;

        while (current != nullptr) {
            Node* copy = new Node(current->val);
            dict[current] = copy;

            current = current->next;
        }

        current = head;
        while (current != nullptr) {
            if (current->next != nullptr) {
                dict[current]->next = dict[current->next];
            }
            if (current->random != nullptr) {
                dict[current]->random = dict[current->random];
            }
            
            current = current->next;  
        }

        return dict[head];  
    }
};
```