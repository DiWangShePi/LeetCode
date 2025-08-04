# 432. All O`one Data Structure

### Description

Design a data structure to store the strings' count with the ability to return the strings with minimum and maximum counts.

Implement the AllOne class:

- AllOne() Initializes the object of the data structure.
- inc(String key) Increments the count of the string key by 1. If key does not exist in the data structure, insert it with count 1.
- dec(String key) Decrements the count of the string key by 1. If the count of key is 0 after the decrement, remove it from the data structure. It is guaranteed that key exists in the data structure before the decrement.
- getMaxKey() Returns one of the keys with the maximal count. If no element exists, return an empty string "".
- getMinKey() Returns one of the keys with the minimum count. If no element exists, return an empty string "".
Note that each function must run in O(1) average time complexity.

### Example 

###### Example I

> Input
> ["AllOne", "inc", "inc", "getMaxKey", "getMinKey", "inc", "getMaxKey", "getMinKey"]
> [[], ["hello"], ["hello"], [], [], ["leet"], [], []]
> Output
> [null, null, null, "hello", "hello", null, "hello", "leet"]
> 
> Explanation
> AllOne allOne = new AllOne();
> allOne.inc("hello");
> allOne.inc("hello");
> allOne.getMaxKey(); // return "hello"
> allOne.getMinKey(); // return "hello"
> allOne.inc("leet");
> allOne.getMaxKey(); // return "hello"
> allOne.getMinKey(); // return "leet"

### Solution

字典加双向链表。链表在每次更新的时候有用，实现O(1)的方式是不必遍历去寻找合适的位置，由于每次只加一，只需要考虑前一个或后一个是否存在，存在就可以直接更新key值对应的count。

```c++
class AllOne {
private:
    struct Node {
        int count;
        unordered_set<string> keys;
    };

    list<Node> nodes; 
    unordered_map<string, list<Node>::iterator> key_to_node;

public:
    AllOne() {}

    void inc(string key) {
        if (key_to_node.find(key) == key_to_node.end()) {
            if (nodes.empty() || nodes.front().count != 1) {
                nodes.push_front({1, {key}});
                key_to_node[key] = nodes.begin();
            } else {
                nodes.front().keys.insert(key);
                key_to_node[key] = nodes.begin();
            }
        } else {
            auto curr = key_to_node[key];
            auto next_it = std::next(curr);  

            int new_count = curr->count + 1;
            if (next_it == nodes.end() || next_it->count != new_count) {
                next_it = nodes.insert(next_it, {new_count, {}});
            }
            next_it->keys.insert(key);
            key_to_node[key] = next_it;

            curr->keys.erase(key);
            if (curr->keys.empty()) {
                nodes.erase(curr);
            }
        }
    }

    void dec(string key) {
        if (key_to_node.find(key) == key_to_node.end()) return;

        auto curr = key_to_node[key];
        if (curr->count == 1) {
            curr->keys.erase(key);
            key_to_node.erase(key);
            if (curr->keys.empty()) nodes.erase(curr);
        } else {
            auto prev_it = prev(curr);
            int new_count = curr->count - 1;

            if (curr == nodes.begin() || prev_it->count != new_count) {
                prev_it = nodes.insert(curr, {new_count, {}});
            }
            prev_it->keys.insert(key);
            key_to_node[key] = prev_it;

            curr->keys.erase(key);
            if (curr->keys.empty()) nodes.erase(curr);
        }
    }

    string getMaxKey() {
        if (nodes.empty()) return "";
        return *nodes.back().keys.begin();
    }

    string getMinKey() {
        if (nodes.empty()) return "";
        return *nodes.front().keys.begin();
    }
};
```
