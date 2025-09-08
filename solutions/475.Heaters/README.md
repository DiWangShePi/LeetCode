# 475. Heaters

### Description

Winter is coming! During the contest, your first job is to design a standard heater with a fixed warm radius to warm all the houses.

Every house can be warmed, as long as the house is within the heater's warm radius range. 

Given the positions of houses and heaters on a horizontal line, return the minimum radius standard of heaters so that those heaters could cover all houses.

Notice that all the heaters follow your radius standard, and the warm radius will be the same.

### Example 

###### Example I

> Input: houses = [1,2,3], heaters = [2]
> Output: 1
> Explanation: The only heater was placed in the position 2, and if we use the radius 1 standard, then all the houses can be warmed.

###### Example II

> Input: houses = [1,2,3,4], heaters = [1,4]
> Output: 1
> Explanation: The two heaters were placed at positions 1 and 4. We need to use a radius 1 standard, then all the houses can be warmed.

###### Example III

> Input: houses = [1,5], heaters = [2]
> Output: 3

### Solution

暴力的做法是遍历每一种可能，将houses能被cover的值更新为最小的，然后遍历一遍找最大的。

```c++
class Solution {
public:
    int findRadius(vector<int>& houses, vector<int>& heaters) {
        sort(houses.begin(), houses.end());
        sort(heaters.begin(), heaters.end());

        vector<int> coverable(houses.size(), -1);
        for (int i = heaters.size() - 1; i > -1; i--) {
            int heater = heaters[i];
            for (int j = 0; j < houses.size(); j++) {
                int cover = abs(heater - houses[j]);
                if (coverable[j] == -1) {
                    coverable[j] = cover;
                    continue;
                }

                if (cover < coverable[j]) coverable[j] = cover;
                else break;
            }
        }

        int an = INT_MIN;
        for (int c : coverable) an = max(an, c);
        return an;
    }
};
```

但是，一个houses要被开销最小的cover，要么是轴上前面的heater，要么是后面的。因此我们可以先排序，再二分找到合适的位置，比较出较小的值。

```c++
class Solution {
public:
    int findRadius(vector<int>& houses, vector<int>& heaters) {
        sort(houses.begin(), houses.end());
        sort(heaters.begin(), heaters.end());

        int ans = 0;
        for (int house : houses) {
            auto it = lower_bound(heaters.begin(), heaters.end(), house);

            int dist1 = INT_MAX, dist2 = INT_MAX;
            if (it != heaters.end()) {
                dist1 = *it - house; 
            }
            if (it != heaters.begin()) {
                dist2 = house - *(it - 1); 
            }
            ans = max(ans, min(dist1, dist2));
        }
        return ans;
    }
};
```
