# 378. Kth Smallest Element in a Sorted Matrix

### Description

Given an n x n matrix where each of the rows and columns is sorted in ascending order, return the kth smallest element in the matrix.

Note that it is the kth smallest element in the sorted order, not the kth distinct element.

You must find a solution with a memory complexity better than O(n2).

### Example 

###### Example I

```
Input: matrix = [[1,5,9],[10,11,13],[12,13,15]], k = 8
Output: 13
Explanation: The elements in the matrix are [1,5,9,10,11,12,13,13,15], and the 8th smallest number is 13
```

###### Example II

```
Input: matrix = [[-5]], k = 1
Output: -5
```

### Solution

1. 全部塞到一个数组里，排序后返回指定位置的数字。

```c++
class Solution {
public:
    int kthSmallest(vector<vector<int>>& matrix, int k) {
        int m = matrix.size();
        int n = matrix[0].size();

        vector<int> an;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                an.push_back(matrix[i][j]);
            }
        }
        sort(an.begin(), an.end());

        return an[k - 1];
    }
};
```

2. 最小堆合并

```c++
struct Element {
    int val;
    int row;
    int col;
    Element(int v, int r, int c) : val(v), row(r), col(c) {}
    bool operator>(const Element& other) const {
        return val > other.val;
    }
};


class Solution {
public:
    int kthSmallest(vector<vector<int>>& matrix, int k) {
        int n = matrix.size();
        priority_queue<Element, vector<Element>, greater<Element>> minHeap;
        
        for (int i = 0; i < n; ++i) {
            minHeap.emplace(matrix[i][0], i, 0);
        }
        
        int result = 0;
        for (int i = 0; i < k; ++i) {
            Element current = minHeap.top();
            minHeap.pop();
            result = current.val;
            
            if (current.col + 1 < n) {
                minHeap.emplace(matrix[current.row][current.col + 1], current.row, current.col + 1);
            }
        }
        
        return result;
    }
};
```

3. 二分查找

```c++
class Solution {
public:
    int kthSmallest(vector<vector<int>>& matrix, int k) {
        int n = matrix.size();
        int left = matrix[0][0];
        int right = matrix[n-1][n-1];
        
        while (left < right) {
            int mid = left + (right - left) / 2;
            int count = 0;
            int j = n - 1;
            
            for (int i = 0; i < n; ++i) {
                while (j >= 0 && matrix[i][j] > mid) {
                    --j;
                }
                count += (j + 1);
            }
            
            if (count < k) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        return left;
    }
};
```