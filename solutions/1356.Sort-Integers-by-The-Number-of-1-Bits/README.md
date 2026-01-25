# 1356. Sort Integers by The Number of 1 Bits

**Tags:** Bitwise operations

### Description

You are given an integer array arr. Sort the integers in the array in ascending order by the number of 1's in their binary representation and in case of two or more integers have the same number of 1's you have to sort them in ascending order.

Return the array after sorting it.

### Example

###### Example I

> Input: arr = [0,1,2,3,4,5,6,7,8]
> Output: [0,1,2,4,8,3,5,6,7]
> Explantion: [0] is the only integer with 0 bits.
> [1,2,4,8] all have 1 bit.
> [3,5,6] have 2 bits.
> [7] has 3 bits.
> The sorted array by bits is [0,1,2,4,8,3,5,6,7]

###### Example II

> Input: arr = [1024,512,256,128,64,32,16,8,4,2,1]
> Output: [1,2,4,8,16,32,64,128,256,512,1024]
> Explantion: All integers have 1 bit in the binary representation, you should just sort them in ascending order.

### Solution

计算一遍所有数字包含的 1 的个数，定义排序算法先比较 1 的个数，再比较数字大小。

```c++
class Solution {
public:
    int get(int x){
        int res = 0;
        while (x) {
            res += (x % 2);
            x /= 2;
        }
        return res;
    }
    vector<int> sortByBits(vector<int>& arr) {
        vector<int> bit(10001, 0);
        for (auto x: arr) {
            bit[x] = get(x);
        }
        sort(arr.begin(), arr.end(), [&](int x, int y){
            if (bit[x] < bit[y]) {
                return true;
            }
            if (bit[x] > bit[y]) {
                return false;
            }
            return x < y;
        });
        return arr;
    }
};
```