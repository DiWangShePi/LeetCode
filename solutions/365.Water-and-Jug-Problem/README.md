# 365. Water and Jug Problem

### Description

You are given two jugs with capacities x liters and y liters. You have an infinite water supply. Return whether the total amount of water in both jugs may reach target using the following operations:

- Fill either jug completely with water.
- Completely empty either jug.
- Pour water from one jug into another until the receiving jug is full, or the transferring jug is empty.

### Example 

###### Example I

```
Input: x = 3, y = 5, target = 4

Output: true

Explanation:

Follow these steps to reach a total of 4 liters:

Fill the 5-liter jug (0, 5).
Pour from the 5-liter jug into the 3-liter jug, leaving 2 liters (3, 2).
Empty the 3-liter jug (0, 2).
Transfer the 2 liters from the 5-liter jug to the 3-liter jug (2, 0).
Fill the 5-liter jug again (2, 5).
Pour from the 5-liter jug into the 3-liter jug until the 3-liter jug is full. This leaves 4 liters in the 5-liter jug (3, 4).
Empty the 3-liter jug. Now, you have exactly 4 liters in the 5-liter jug (0, 4).
```

###### Example II

```
Input: x = 2, y = 6, target = 5

Output: false
```

###### Example III

```
Input: x = 1, y = 2, target = 3

Output: true

Explanation: Fill both jugs. The total amount of water in both jugs is equal to 3 now.
```

### Solution

我们认为，每次操作只会让两只桶的总水量增加x，增加y，减少x或者减少y。由此，我们的目标可以变为，找寻整数a和b，使得

$$
ax + by = z
$$

只要满足z<=x+y(不能大过水壶总量)，且存在这样一对a和b，我们的目标就是可以达成的。同时，贝祖定理告诉我们，ax+by=z 有解当且仅当 z 是 x,y 的最大公约数的倍数。因此我们只需要找到 x,y 的最大公约数并判断 z 是否是它的倍数即可。

```c++
class Solution {
public:
    bool canMeasureWater(int x, int y, int target) {
        if (x + y < target) return false;
        if (x == 0 || y == 0) return target == 0 || x + y == target;
        return target % gcd(x, y) == 0;
    }
};
```

> 当然，我们也可以用深度优先搜索
