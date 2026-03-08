# 3861. Minimum Capacity Box

### Description

You are given an integer array capacity, where capacity[i] represents the capacity of the ith box, and an integer itemSize representing the size of an item.

The ith box can store the item if capacity[i] >= itemSize.

Return an integer denoting the index of the box with the minimum capacity that can store the item. If multiple such boxes exist, return the smallest index.

If no box can store the item, return -1.

### Example

###### Example I

> Input: capacity = [1,5,3,7], itemSize = 3
> Output: 2
> Explanation:
> The box at index 2 has a capacity of 3, which is the minimum capacity that can store the item. Thus, the answer is 2.

###### Example II

> Input: capacity = [3,5,4,3], itemSize = 2
> Output: 0
> Explanation:
> The minimum capacity that can store the item is 3, and it appears at indices 0 and 3. Thus, the answer is 0.

###### Example III

> Input: capacity = [4], itemSize = 5
> Output: -1
> Explanation:
> No box has enough capacity to store the item, so the answer is -1.

### Solution

遍历一遍，记录最优值。

```c++
class Solution {
public:
    int minimumIndex(vector<int>& capacity, int itemSize) {
        int ani = 0, anc = INT_MAX;
        for (int i = 0; i < capacity.size(); i++) {
            int c = capacity[i];
            if (c >= itemSize) {
                if (c < anc) {
                    anc = c;
                    ani = i;
                }
            }
        }
        return anc == INT_MAX ? -1 : ani;
    }
};
```
