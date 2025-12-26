# 1385. Find the Distance Value Between Two Arrays

**Tags:** Binary Search

### Description

Given two integer arrays arr1 and arr2, and the integer d, return the distance value between the two arrays.

The distance value is defined as the number of elements arr1[i] such that there is not any element arr2[j] where |arr1[i]-arr2[j]| <= d.

### Example

###### Example I

> Input: arr1 = [4,5,8], arr2 = [10,9,1,8], d = 2
> Output: 2
> Explanation: 
> For arr1[0]=4 we have: 
> |4-10|=6 > d=2 
> |4-9|=5 > d=2 
> |4-1|=3 > d=2 
> |4-8|=4 > d=2 
> For arr1[1]=5 we have: 
> |5-10|=5 > d=2 
> |5-9|=4 > d=2 
> |5-1|=4 > d=2 
> |5-8|=3 > d=2
> For arr1[2]=8 we have:
> |8-10|=2 <= d=2
> |8-9|=1 <= d=2
> |8-1|=7 > d=2
> |8-8|=0 <= d=2

###### Example II

> Input: arr1 = [1,4,2,3], arr2 = [-4,-3,6,10,20,30], d = 3
> Output: 2

###### Example III

> Input: arr1 = [2,1,100,3], arr2 = [-5,-2,10,-3,7], d = 6
> Output: 1

### Solution

对 arr2 进行排序。对 arr1 中的每一个值，二分查找比他大和比他小的数字，检查是否满足条件。

```c++
class Solution {
public:
    int findTheDistanceValue(vector<int>& arr1, vector<int>& arr2, int d) {
        sort(arr2.begin(), arr2.end());
        int count = 0;
        for (int val : arr1) {
            int p = find(arr2, val);
            bool ok = true;
            if (p < arr2.size()) ok &= (arr2[p] - val > d);
            if (p - 1 > -1 && p - 1 <= arr2.size()) ok &= (val - arr2[p - 1] > d);
            count += ok;
        }
        return count;
    }

private:
    int find(vector<int>& arr2, int t) {
        int l = 0, r = arr2.size() - 1;
        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (arr2[mid] == t) return mid;
            else if (arr2[mid] > t) r = mid - 1;
            else l = mid + 1;
        }
        return l;
    }
};
```
