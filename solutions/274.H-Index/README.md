# 274. H-Index

### Description

Given an array of integers citations where citations[i] is the number of citations a researcher received for their ith paper, return the researcher's h-index.

According to the definition of h-index on Wikipedia: The h-index is defined as the maximum value of h such that the given researcher has published at least h papers that have each been cited at least h times.

### Solution

###### Example I:

```
Since the researcher has 3 papers with at least 3 citations each and the remaining two with no more than 3 citations each, their h-index is 3.
```

###### Example II:

```
Input: citations = [1,3,1]
Output: 1
```

### Solution

一开始我想用小顶堆解决这个问题：如果当前堆顶的元素值小于等于堆的大小，即代表当前值合法。在当前值与上一个保存的当前值不一致时更新。

```c++
class Solution {
public:
    int hIndex(vector<int>& citations) {
        priority_queue<int, vector<int>, greater<int>> minHeap;
        for (int val : citations) minHeap.push(val);

        int result = 0;
        while (!minHeap.empty()) {
            if (result != minHeap.top()) {
                if (minHeap.top() <= minHeap.size()) {
                    result = minHeap.top();
                } else break;
            }
            minHeap.pop();
        }
        return result;
    }
};
```

但发现这一思路无法解决如`[100]`这样的corner case，因此转而诉诸另一个思路：先将数组从大到小排序，若当前数组的值大于其位置i，即为合法值，寻找最后一个合法的i。

```c++
class Solution {
public:
    int hIndex(vector<int>& citations) {
        sort(citations.begin(), citations.end(), greater<int>());
        int h = 0;
        for (int i = 0; i < citations.size(); ++i) {
            if (citations[i] >= i + 1)
                ++h;
            else
                break;
        }
        return h;
    }
};
```
