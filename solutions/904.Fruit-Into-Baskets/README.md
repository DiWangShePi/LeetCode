# 904. Fruit Into Baskets

**Tags:** Sliding Window, Dict

### Description

You are visiting a farm that has a single row of fruit trees arranged from left to right. The trees are represented by an integer array fruits where fruits[i] is the type of fruit the ith tree produces.

You want to collect as much fruit as possible. However, the owner has some strict rules that you must follow:

- You only have two baskets, and each basket can only hold a single type of fruit. There is no limit on the amount of fruit each basket can hold.
- Starting from any tree of your choice, you must pick exactly one fruit from every tree (including the start tree) while moving to the right. The picked fruits must fit in one of your baskets.
- Once you reach a tree with fruit that cannot fit in your baskets, you must stop.
Given the integer array fruits, return the maximum number of fruits you can pick.

### Example

###### Example I

> Input: fruits = [1,2,1]
> Output: 3
> Explanation: We can pick from all 3 trees.

###### Example II

> Input: fruits = [0,1,2,2]
> Output: 3
> Explanation: We can pick from trees [1,2,2].
> If we had started at the first tree, we would only pick from trees [0,1].

###### Example III

> Input: fruits = [1,2,3,2,2]
> Output: 4
> Explanation: We can pick from trees [2,3,2,2].
> If we had started at the first tree, we would only pick from trees [1,2].

### Solution

用字典记录拿到的水果的个数，字典的大小不能超过2，超过时就缩减右边界的长度至满足条件。记录符合要求的最短的子序列长度。

```c++
class Solution {
public:
    int totalFruit(vector<int>& fruits) {
        unordered_map<int, int> bucket;
        int an = 0, l = 0, n = fruits.size();

        for (int i = 0; i < n; i++) {
            int f = fruits[i];
            if (bucket.count(f) == 0) bucket[f] = 0;
            bucket[f]++;

            while (bucket.size() > 2) {
                bucket[fruits[l]]--;
                if (bucket[fruits[l]] == 0) bucket.erase(fruits[l]);

                l++;
            }

            an = max(an, i - l + 1);
        }
        return max(an, n - l);
    }
};
```
