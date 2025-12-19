# 825. Friends Of Appropriate Ages

**Tags:** Counting Sort, Prefix Sum, Sliding Window

### Description

There are n persons on a social media website. You are given an integer array ages where ages[i] is the age of the ith person.

A Person x will not send a friend request to a person y (x != y) if any of the following conditions is true:

- age[y] <= 0.5 * age[x] + 7
- age[y] > age[x]
- age[y] > 100 && age[x] < 100
Otherwise, x will send a friend request to y.

Note that if x sends a request to y, y will not necessarily send a request to x. Also, a person will not send a friend request to themself.

Return the total number of friend requests made.

### Example

###### Example I

> Input: ages = [16,16]
> Output: 2
> Explanation: 2 people friend request each other.

###### Example II

> Input: ages = [16,17,18]
> Output: 2
> Explanation: Friend requests are made 17 -> 16, 18 -> 17.

###### Example III

> Input: ages = [20,30,100,110,120]
> Output: 3
> Explanation: Friend requests are made 110 -> 100, 120 -> 110, 120 -> 100.

### Solution

由于年龄的范围不大，所以我们可以用计数排序获得每个年龄的人数，再用前缀和计算指定人数的累计总数。

```c++
class Solution {
public:
    int numFriendRequests(vector<int>& ages) {
        vector<int> count(121, 0);
        for (int age : ages) count[age]++;
        vector<int> sums(121, 0);
        for (int i = 1; i <= 120; i++) {
            sums[i] = sums[i - 1] + count[i];
        }

        int an = 0;
        for (int x = 15; x <= 120; x++) {
            if (count[x] == 0) continue;
            int y = 0.5 * x + 8;
            an += count[x] * (sums[x] - sums[y - 1] - 1);
        }
        return an;
    }
};
```

或者我们也可以用滑动窗口。先对 ages 进行排序，然后对于每个 age，找到左边界和右边界。由于 age 是增加的，那么左边界和右边界也都是只会增加的。

```c++
class Solution {
public:
    int numFriendRequests(vector<int>& ages) {
        sort(ages.begin(), ages.end());
        int l = 0, r = 0, ans = 0;
        for (int age : ages) {
            if (age < 15) continue;
            while (ages[l] <= 0.5 * age + 7) l++;
            while (r < ages.size() - 1 && ages[r + 1] <= age) r++;
            ans += r - l;
        }
        return ans;
    }
};
```
