# 480. Sliding Window Median

**Tags:** Heap, Lazy Deletion

### Description

The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value. So the median is the mean of the two middle values.

- For examples, if arr = [2,3,4], the median is 3.
- For examples, if arr = [1,2,3,4], the median is (2 + 3) / 2 = 2.5.
You are given an integer array nums and an integer k. There is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position.

Return the median array for each window in the original array. Answers within 10-5 of the actual value will be accepted.

### Example 

###### Example I

> Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
> Output: [1.00000,-1.00000,-1.00000,3.00000,5.00000,6.00000]
> Explanation: 
> Window position                Median
> ---------------                -----
> [1  3  -1] -3  5  3  6  7        1
>  1 [3  -1  -3] 5  3  6  7       -1
>  1  3 [-1  -3  5] 3  6  7       -1
>  1  3  -1 [-3  5  3] 6  7        3
>  1  3  -1  -3 [5  3  6] 7        5
>  1  3  -1  -3  5 [3  6  7]       6

###### Example II

> Input: nums = [1,2,3,4,2,3,1,4,2], k = 3
> Output: [2.00000,3.00000,3.00000,3.00000,2.00000,3.00000,2.00000]

### Solution

一个大顶堆，一个小顶堆。懒惰删除处理堆不支持删除非堆顶元素。

> 当年DSAA课上的作业，写了这么久的题终于遇到了

```c++
class DualHeap {
private:
    priority_queue<int> small;
    priority_queue<int, vector<int>, greater<int>> large;
    unordered_map<int, int> delayed;
    int smallSize;
    int largeSize;
    int k;

    template<typename T>
    void prune(T& heap) {
        while (!heap.empty()) {
            int num = heap.top();
            if (delayed.find(num) != delayed.end() && delayed[num] > 0) {
                delayed[num]--;
                heap.pop();
            } else {
                break;
            }
        }
    }

    void makeBalance() {
        if (smallSize > largeSize + 1) {
            large.push(small.top());
            small.pop();
            smallSize--;
            largeSize++;
            prune(small);
        } else if (largeSize > smallSize) {
            small.push(large.top());
            large.pop();
            largeSize--;
            smallSize++;
            prune(large);
        }
    }

public:
    DualHeap(int windowSize) : k(windowSize), smallSize(0), largeSize(0) {}

    void insert(int num) {
        if (small.empty() || num <= small.top()) {
            small.push(num);
            smallSize++;
        } else {
            large.push(num);
            largeSize++;
        }
        makeBalance();
    }

    void remove(int num) {
        delayed[num]++;
        if (num <= small.top()) {
            smallSize--;
        } else {
            largeSize--;
        }
        makeBalance();
        prune(small);
        prune(large);
    }

    double getMedian() {
        if (k % 2 == 1) {
            return small.top();
        } else {
            return ((double)small.top() + large.top()) / 2.0;
        }
    }
};

class Solution {
public:
    vector<double> medianSlidingWindow(vector<int>& nums, int k) {
        vector<double> result;
        DualHeap dh(k);
        
        for (int i = 0; i < k; i++) {
            dh.insert(nums[i]);
        }
        result.push_back(dh.getMedian());
        
        for (int i = k; i < nums.size(); i++) {
            dh.remove(nums[i - k]);
            dh.insert(nums[i]);
            result.push_back(dh.getMedian());
        }
        
        return result;
    }
};
```
