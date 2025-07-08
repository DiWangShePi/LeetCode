# 398. Random Pick Index

### Description

Given an integer array nums with possible duplicates, randomly output the index of a given target number. You can assume that the given target number must exist in the array.

Implement the Solution class:

- Solution(int[] nums) Initializes the object with the array nums.
- int pick(int target) Picks a random index i from nums where nums[i] == target. If there are multiple valid i's, then each index should have an equal probability of returning.

### Example 

###### Example I

> Input
> ["Solution", "pick", "pick", "pick"]
> [[[1, 2, 3, 3, 3]], [3], [1], [3]]
> Output
> [null, 4, 0, 2]
> Explanation
> Solution solution = new Solution([1, 2, 3, 3, 3]);
> solution.pick(3); // It should return either index 2, 3, or 4 randomly. Each index should have equal probability of returning.
> solution.pick(1); // It should return 0. Since in the array only nums[0] is equal to 1.
> solution.pick(3); // It should return either index 2, 3, or 4 randomly. Each index should have equal probability of returning.

### Solution

先遍历一遍数组，获取每个数组的下标组成的列表，用洗牌算法shuffle这个列表，每次读取时轮流返回

```c++
class Solution {
    unordered_map<int, vector<int>> position;
    unordered_map<int, int> last_position;

public:
    Solution(vector<int>& nums) {
        for (int i = 0; i < nums.size(); i++){
            int num = nums[i];
            if (position.count(num) == 0) {
                position[num] = {};
                last_position[num] = 0;
            }
            position[num].push_back(i);
        }

        for (auto iter = position.begin(); iter != position.end(); ++iter)
            shuffle(iter->second);
    }
    
    int pick(int target) {
        int p = position[target][last_position[target]];
        last_position[target] += 1;
        last_position[target] %= position[target].size();
        return p;
    }

private:
    void shuffle(vector<int>& array) {
        for (int i = array.size() - 1; i > -1; i--) 
            swap(array[i], array[rand() % (i + 1)]);
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(nums);
 * int param_1 = obj->pick(target);
 */
```

> 按照官方解法的描述，不shuffle也是可以的。奇妙。、
