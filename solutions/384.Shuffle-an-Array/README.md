# 384. Shuffle an Array

### Description

Given an integer array nums, design an algorithm to randomly shuffle the array. All permutations of the array should be equally likely as a result of the shuffling.

Implement the Solution class:

- Solution(int[] nums) Initializes the object with the integer array nums.
- int[] reset() Resets the array to its original configuration and returns it.
- int[] shuffle() Returns a random shuffling of the array.

### Example 

###### Example I

```
Input
["Solution", "shuffle", "reset", "shuffle"]
[[[1, 2, 3]], [], [], []]
Output
[null, [3, 1, 2], [1, 2, 3], [1, 3, 2]]

Explanation
Solution solution = new Solution([1, 2, 3]);
solution.shuffle();    // Shuffle the array [1,2,3] and return its result.
                       // Any permutation of [1,2,3] must be equally likely to be returned.
                       // Example: return [3, 1, 2]
solution.reset();      // Resets the array back to its original configuration [1,2,3]. Return [1, 2, 3]
solution.shuffle();    // Returns the random shuffling of array [1,2,3]. Example: return [1, 3, 2]
```

### Solution

初始化时保留一个副本，reset时返回副本，shuffle时使用洗牌算法（Knuth-Shuffle）生成新的。

```c++
class Solution {
    vector<int> array;

public:
    Solution(vector<int>& nums) {
        array = vector<int>(nums.begin(), nums.end());
    }
    
    vector<int> reset() {
        return array;
    }
    
    vector<int> shuffle() {
        vector<int> copy(array.begin(), array.end());
        for (int i = array.size() - 1; i > -1; i--) 
            swap(copy[i], copy[rand() % (i + 1)]);
        return copy;
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(nums);
 * vector<int> param_1 = obj->reset();
 * vector<int> param_2 = obj->shuffle();
 */
```
