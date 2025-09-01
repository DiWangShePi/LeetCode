# 460. LFU Cache

### Description

Design and implement a data structure for a Least Frequently Used (LFU) cache.

Implement the LFUCache class:

- LFUCache(int capacity) Initializes the object with the capacity of the data structure.
i- nt get(int key) Gets the value of the key if the key exists in the cache. Otherwise, returns -1.
- void put(int key, int value) Update the value of the key if present, or inserts the key if not already present. When the cache reaches its capacity, it should invalidate and remove the least frequently used key before inserting a new item. For this problem, when there is a tie (i.e., two or more keys with the same frequency), the least recently used key would be invalidated.
To determine the least frequently used key, a use counter is maintained for each key in the cache. The key with the smallest use counter is the least frequently used key.

When a key is first inserted into the cache, its use counter is set to 1 (due to the put operation). The use counter for a key in the cache is incremented either a get or put operation is called on it.

The functions get and put must each run in O(1) average time complexity.

### Example 

###### Example I

> Input
> ["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get"]
> [[2], [1, 1], [2, 2], [1], [3, 3], [2], [3], [4, 4], [1], [3], [4]]
> Output
> [null, null, null, 1, null, -1, 3, null, -1, 3, 4]
> 
> Explanation
> // cnt(x) = the use counter for key x
> // cache=[] will show the last used order for tiebreakers (leftmost element is  most recent)
> LFUCache lfu = new LFUCache(2);
> lfu.put(1, 1);   // cache=[1,_], cnt(1)=1
> lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
> lfu.get(1);      // return 1
>                  // cache=[1,2], cnt(2)=1, cnt(1)=2
> lfu.put(3, 3);   // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
>                  // cache=[3,1], cnt(3)=1, cnt(1)=2
> lfu.get(2);      // return -1 (not found)
> lfu.get(3);      // return 3
>                  // cache=[3,1], cnt(3)=2, cnt(1)=2
> lfu.put(4, 4);   // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
>                  // cache=[4,3], cnt(4)=1, cnt(3)=2
> lfu.get(1);      // return -1 (not found)
> lfu.get(3);      // return 3
>                  // cache=[3,4], cnt(4)=1, cnt(3)=3
> lfu.get(4);      // return 4
>                  // cache=[4,3], cnt(4)=2, cnt(3)=3

### Solution

用字典存储key到value的映射，用链表排列用过的节点，每次更新的时候，将节点的频率更新，移动到新的位置。一个节点中可以存储多个同一频率的key-value，为了O(1)的知道哪一个映射出现的最早，再用一个链表来维护，新加入的值在队尾，队首就是该频率下最早出现的值。

```c++
class LFUCache {
private:
    int capacity;
    int minFreq;
    unordered_map<int, list<pair<int, int>>::iterator> keyToNode;
    unordered_map<int, list<pair<int, int>>> freqToList;
    unordered_map<int, int> keyToFreq;

public:
    LFUCache(int capacity) : capacity(capacity), minFreq(0) {}
    
    int get(int key) {
        if (capacity == 0) return -1;
        if (keyToNode.find(key) == keyToNode.end()) {
            return -1;
        }
        
        int value = keyToNode[key]->second;
        int freq = keyToFreq[key];
        
        freqToList[freq].erase(keyToNode[key]);
        
        if (freqToList[freq].empty()) {
            freqToList.erase(freq);
            if (minFreq == freq) {
                minFreq++;
            }
        }
        
        freq++;
        freqToList[freq].push_front({key, value});

        keyToNode[key] = freqToList[freq].begin();
        keyToFreq[key] = freq;
        
        return value;
    }
    
    void put(int key, int value) {
        if (capacity == 0) return;
        
        if (keyToNode.find(key) != keyToNode.end()) {
            int freq = keyToFreq[key];
            
            freqToList[freq].erase(keyToNode[key]);
            
            if (freqToList[freq].empty()) {
                freqToList.erase(freq);
                if (minFreq == freq) {
                    minFreq++;
                }
            }
            
            freq++;
            freqToList[freq].push_front({key, value});
            
            keyToNode[key] = freqToList[freq].begin();
            keyToFreq[key] = freq;
            
            return;
        }
        
        if (keyToNode.size() >= capacity) {
            auto& minFreqList = freqToList[minFreq];
            int keyToRemove = minFreqList.back().first;
            
            keyToNode.erase(keyToRemove);
            keyToFreq.erase(keyToRemove);
            minFreqList.pop_back();
            
            if (minFreqList.empty()) {
                freqToList.erase(minFreq);
            }
        }
        
        freqToList[1].push_front({key, value});
        keyToNode[key] = freqToList[1].begin();
        keyToFreq[key] = 1;
        minFreq = 1; 
    }
};

/**
 * Your LFUCache object will be instantiated and called as such:
 * LFUCache* obj = new LFUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */
```
