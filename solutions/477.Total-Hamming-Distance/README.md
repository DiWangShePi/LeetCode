# 477. Total Hamming Distance

**Tags:** Bitwise

### Description

The Hamming distance between two integers is the number of positions at which the corresponding bits are different.

Given an integer array nums, return the sum of Hamming distances between all the pairs of the integers in nums.

### Example

###### Example I

> Input: nums = [4,14,2]
> Output: 6
> Explanation: In binary representation, the 4 is 0100, 14 is 1110, and 2 is 0010 (just
> showing the four bits relevant in this case).
> The answer will be:
> HammingDistance(4, 14) + HammingDistance(4, 2) + HammingDistance(14, 2) = 2 + 2 + 2 = 6.

###### Example II

> Input: nums = [4,14,4]
> Output: 4

### Solution

看懂了题意，先尝试一下暴力解法

```c++
class Solution {
public:
    int totalHammingDistance(vector<int>& nums) {
        int an = 0;
        for (int i = 0; i < nums.size(); i++) {
            for (int j = i; j < nums.size(); j++) {
                an += hammingDistance(nums[i], nums[j]);
            }
        }
        return an;
    }

private:
    int hammingDistance(int x, int y) {
        int c = x ^ y;
        int count = 0;
        for (int i = 31; i > -1; i--) {
            if (c & ( 1 << i)) count++;
        }
        return count;
    }
};
```

随后我们考虑优雅一点的解法，对于数字的每一位i，假定存在c个数字该位为1，则有n-c个数字该位为0。
那么在这一位上，所有数字的汉明距离为：c(n - c)

```c++
class Solution {
public:
    int totalHammingDistance(vector<int>& nums) {
        int an = 0, n = nums.size();
        for (int i = 0; i < 30; i++) {
            int count = 0;
            for (int num : nums) 
                count += (num >> i) & 1;

            an += count * (n - count);
        }
        return an;
    }
};
```
