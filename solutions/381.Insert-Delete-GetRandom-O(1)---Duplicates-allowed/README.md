# 381. Insert Delete GetRandom O(1) - Duplicates allowed

### Description

RandomizedCollection is a data structure that contains a collection of numbers, possibly duplicates (i.e., a multiset). It should support inserting and removing specific elements and also reporting a random element.

Implement the RandomizedCollection class:

- RandomizedCollection() Initializes the empty RandomizedCollection object.
- bool insert(int val) Inserts an item val into the multiset, even if the item is already present. Returns true if the item is not present, false otherwise.
- bool remove(int val) Removes an item val from the multiset if present. Returns true if the item is present, false otherwise. Note that if val has multiple occurrences in the multiset, we only remove one of them.
- int getRandom() Returns a random element from the current multiset of elements. The probability of each element being returned is linearly related to the number of the same values the multiset contains.

You must implement the functions of the class such that each function works on average O(1) time complexity.

Note: The test cases are generated such that getRandom will only be called if there is at least one item in the RandomizedCollection.

### Example

###### Example I

```
Input
["RandomizedCollection", "insert", "insert", "insert", "getRandom", "remove", "getRandom"]
[[], [1], [1], [2], [], [1], []]
Output
[null, true, false, true, 2, true, 1]

Explanation
RandomizedCollection randomizedCollection = new RandomizedCollection();
randomizedCollection.insert(1);   // return true since the collection does not contain 1.
                                  // Inserts 1 into the collection.
randomizedCollection.insert(1);   // return false since the collection contains 1.
                                  // Inserts another 1 into the collection. Collection now contains [1,1].
randomizedCollection.insert(2);   // return true since the collection does not contain 2.
                                  // Inserts 2 into the collection. Collection now contains [1,1,2].
randomizedCollection.getRandom(); // getRandom should:
                                  // - return 1 with probability 2/3, or
                                  // - return 2 with probability 1/3.
randomizedCollection.remove(1);   // return true since the collection contains 1.
                                  // Removes 1 from the collection. Collection now contains [1,2].
randomizedCollection.getRandom(); // getRandom should return 1 or 2, both equally likely.
```

### Solution

在上一题的基础上，将字典指向的单个字符改为记录下标的数组。动态数组用于存储所有元素，保证随机访问的效率；哈希表则记录每个元素在数组中的所有位置（使用集合存储索引），以支持快速查找和删除。插入时，直接在数组末尾添加元素，并更新哈希表；删除时，通过交换目标元素和数组末尾元素的位置，再弹出末尾元素，同时更新哈希表中两者的位置信息。随机获取时，直接通过数组的随机下标访问。

```c++
class RandomizedCollection {
    vector<int> values;
    unordered_map<int, unordered_set<int>> dict; 

public:
    RandomizedCollection() {}
    
    bool insert(int val) {
        bool exists = dict.count(val) > 0;

        values.push_back(val);
        dict[val].insert(values.size() - 1);
        
        return !exists;
    }
    
    bool remove(int val) {
        if (!dict.count(val)) return false;

        int lastPos = values.size() - 1;
        int valPos = *dict[val].begin();
        int lastVal = values.back();

        values[valPos] = lastVal;
        dict[val].erase(valPos);

        if (valPos != lastPos) {
            dict[lastVal].erase(lastPos);
            dict[lastVal].insert(valPos);
        }

        values.pop_back();

        if (dict[val].empty()) dict.erase(val);

        return true;
    }
    
    int getRandom() {
        int po = rand() % values.size();
        return values[po];
    }
};
```
