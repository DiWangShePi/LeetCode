# 128. Longest Consecutive Sequence

### Description

Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.

You must write an algorithm that runs in O(n) time.

### Solution

定义一个新的类，Node，包含值和连续值两个变量。

遍历给定数组，对于每一个数字，新建Node并将它加入到一个字典中。同时，检查字典中是否存在当前值减一或者加一的元素，若存在，将其连续值设定为左右之和加一。

遍历过程中记录最大连续值。

> 这样写完，不要这个类其实也可以

### Implementation

###### c++

```c++
class Solution {
public:
    int longestConsecutive(vector<int>& nums) {
            if (nums.empty()) return 0;
    
            unordered_map<int, int> map;
            int maxLength = 0;
            
            for (int num : nums) {
                if (map.find(num) != map.end()) continue; 
                
                int left = map.find(num - 1) != map.end() ? map[num - 1] : 0;
                int right = map.find(num + 1) != map.end() ? map[num + 1] : 0;
                
                int currentLength = left + 1 + right;
                maxLength = max(maxLength, currentLength);
                map[num] = currentLength;
                
                map[num - left] = currentLength;  
                map[num + right] = currentLength; 
            }
            
            return maxLength;
    }
};
```

> 奇妙的是，这一题你用sort之后能比其他人都快
