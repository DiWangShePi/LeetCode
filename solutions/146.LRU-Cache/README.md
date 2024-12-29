# 146. LRU Cache

### Description

Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.

Implement the LRUCache class:

- LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
- int get(int key) Return the value of the key if the key exists, otherwise return -1.
- void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the number of keys exceeds the capacity from this operation, evict the least recently used key.
The functions get and put must each run in O(1) average time complexity.

### Solution

哈希表和双向链表，将每个新访问的元素放到开头，达到限制时从尾部获取并删除。

### Implementation

###### c++

```c++
#include <unordered_map>
using namespace std;

class LRUCache {
public:
    int Capacity;
    int length;

    struct Node {
        int key;
        int val;
        Node* before;
        Node* after;
    };

    Node* Head;
    Node* Tail;
    unordered_map<int, Node*> dict;

    LRUCache(int capacity) {
        Capacity = capacity;
        length = 0;

        Head = new Node();
        Tail = new Node();

        Head->after = Tail;
        Tail->before = Head;
    }

    ~LRUCache() {
        Node* current = Head;
        while (current) {
            Node* next = current->after;
            delete current;
            current = next;
        }
    }

    int get(int key) {
        if (dict.find(key) != dict.end()) {
            Node* current = dict[key];
            del(current);
            insert(current);
            dict[key] = current;
            return current->val;
        }
        return -1;
    }

    void put(int key, int value) {
        if (dict.count(key) != 0) {
            Node* current = dict[key];
            current->val = value;

            del(current);
            insert(current);
            dict[key] = current;
            return;
        }

        if (length < Capacity) {
            length++;
        } else {
            Node* toRemove = Tail->before;
            del(toRemove);
        }

        Node* current = new Node();
        current->key = key;
        current->val = value;
        dict[key] = current;

        insert(current);
    }

private:
    void del(Node* current) {
        dict.erase(current->key);
        current->before->after = current->after;
        current->after->before = current->before;
    }

    void insert(Node* current) {
        current->after = Head->after;
        Head->after->before = current;
        current->before = Head;
        Head->after = current;
    }
};
```

> 不太理解为什么不删除原本dict中的元素会出错，指针不是没有变吗？只变了对象结构体中部分元素的值啊

> 更优秀的代码

```c++
#define NO_SAN __attribute__((no_sanitize("undefined", "address", "coverage", "thread")))
#define INL __attribute__((always_inline))
#define HOT __attribute__((hot))
#define INL_ATTR noexcept INL NO_SAN HOT
#define OUTL_ATTR noexcept NO_SAN HOT
#define LAM_ATTR INL NO_SAN HOT noexcept

#pragma GCC diagnostic ignored "-Wshift-op-parentheses"

constexpr uint MAX_STORAGE_QW = 7u << 21;
uint64_t storage[MAX_STORAGE_QW];
uint storage_used = 0;

void* operator new(std::size_t sz) {
    if (sz == 0)
        ++sz;
 
    const uint used = storage_used;
    storage_used += sz + 15u >> 4 << 1;
    // assert(storage_used < MAX_STORAGE_QW);
    return storage + used;
}
 
void* operator new[](std::size_t sz) {
    if (sz == 0)
        ++sz;
 
    const uint used = storage_used;
    storage_used += sz + 15u >> 4 << 1;
    // assert(storage_used < MAX_STORAGE_QW);
    return storage + used;
}
 
void operator delete(void* ptr) noexcept {
}
 
void operator delete(void* ptr, std::size_t size) noexcept {
}
 
void operator delete[](void* ptr) noexcept {
}
 
void operator delete[](void* ptr, std::size_t size) noexcept {
}

class LRUCache {
private:
    typedef uint16_t ref_t;
    static constexpr ref_t MAXCAP = 3000, NONE = USHRT_MAX, MAXV = 10'000;
    static constexpr uint8_t KSH = 17;
    static constexpr uint VMASK = (1u << KSH) - 1u;

    struct entry_t {
        uint kv;
        ref_t prev, next;
    };

    static entry_t storage[MAXCAP];
    static ref_t index[MAXV+1u], used, cap, head;

    static void insbefore(const ref_t i, const ref_t next) INL_ATTR {
        if (next == NONE) {
            storage[i].next = storage[i].prev = i;
        } else if (storage[next].next == next) {
            storage[i].next = storage[i].prev = next;
            storage[next].next = storage[next].prev = i;
        } else {
            const ref_t prev = storage[next].prev;
            storage[i].prev = prev;
            storage[next].prev = i;
            storage[i].next = next;
            storage[prev].next = i;
        }
    }

    static void makehead(const ref_t i) INL_ATTR {
        if (head != NONE) {
            const ref_t prev = storage[i].prev, next = storage[i].next;
            storage[next].prev = prev;
            storage[prev].next = next;
            insbefore(i, head);
        }
        head = i;
    }

public:
    LRUCache(const uint c) INL_ATTR {
        cap = c;
        fill(index, index + MAXV, NONE);
    }
    
    static int get(const uint key) OUTL_ATTR {
        // cout << "Get " << key << ':';
        const ref_t i = index[key];
        // if (i == NONE) cout << "NONE" << endl;
        if (i == NONE) return -1;
        if (head != i) makehead(i);
        // cout << (storage[i].kv & VMASK) << endl;
        return storage[i].kv & VMASK;
    }
    
    static void put(const uint key, const uint value) OUTL_ATTR {
        // cout << "Put " << key << ':' << value << endl;
        ref_t i;
        if (i = index[key]; i == NONE) {
            if (used < cap) {
                i = used++;
                // cout << "Allocated " << i << endl;
                insbefore(i, head);
                head = i;
            } else {
                head = i = storage[head].prev;
                index[storage[i].kv >> KSH] = NONE;
            }
        } else if (head != i)
            makehead(i);
        storage[i].kv = (key << KSH) + value;
        index[key] = i;
        // cout << "Now:";
        // ref_t j = head;
        // do {
        //     cout << ' ' << (storage[j].kv >> KSH) << ':' << (storage[j].kv & VMASK);
        //     j = storage[j].next;
        // } while (j != head);
        // cout << endl;
    }

    ~LRUCache() INL_ATTR {
        head = NONE;
        used = 0;
    }
};

LRUCache::entry_t LRUCache::storage[MAXCAP];
LRUCache::ref_t LRUCache::index[MAXV+1u], LRUCache::used, LRUCache::cap, LRUCache::head = NONE;

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */

auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
```
