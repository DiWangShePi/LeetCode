# 295. Find Median from Data Stream

### Description

The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value, and the median is the mean of the two middle values.

- For example, for arr = [2,3,4], the median is 3.
- For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.
Implement the MedianFinder class:

- MedianFinder() initializes the MedianFinder object.
- void addNum(int num) adds the integer num from the data stream to the data structure.
- double findMedian() returns the median of all elements so far. Answers within 10-5 of the actual answer will be accepted.

### Example 

###### Example I:

```
Input
["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
[[], [1], [2], [], [3], []]
Output
[null, null, null, 1.5, null, 2.0]

Explanation
MedianFinder medianFinder = new MedianFinder();
medianFinder.addNum(1);    // arr = [1]
medianFinder.addNum(2);    // arr = [1, 2]
medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
medianFinder.addNum(3);    // arr[1, 2, 3]
medianFinder.findMedian(); // return 2.0
```

### Solution

一个大顶堆，一个小顶堆。维护平衡使得两者相等或者一个比另外的多一个。

> 小顶堆比大顶堆多一个

```c++
class MedianFinder {
    priority_queue<double> max_heap;
    priority_queue<double, vector<double>, greater<double>> min_heap;
public:
    MedianFinder() {}
    
    void addNum(int num) {
        if (min_heap.size() == max_heap.size()) {
            if (min_heap.empty()) min_heap.push(num);
            else {
                if (num < max_heap.top()) {
                    int temp = max_heap.top();
                    max_heap.pop();

                    max_heap.push(num);
                    min_heap.push(temp);
                } else {
                    min_heap.push(num);
                }
            }
        }
        else {
            if (num > min_heap.top()) {
                int temp = min_heap.top();
                min_heap.pop();

                max_heap.push(temp);
                min_heap.push(num);
            } else {
                max_heap.push(num);
            }
        }
    }
    
    double findMedian() {
        if (min_heap.size() == max_heap.size()) return (min_heap.top() + max_heap.top()) / 2;
        else return min_heap.top();
    }
};

/**
 * Your MedianFinder object will be instantiated and called as such:
 * MedianFinder* obj = new MedianFinder();
 * obj->addNum(num);
 * double param_2 = obj->findMedian();
 */
```

> 大顶堆比小顶堆多一个

```c++
class MedianFinder {
    priority_queue<double> max_heap;
    priority_queue<double, vector<double>, greater<double>> min_heap;
public:
    MedianFinder() {}
    
    void addNum(int num) {
        if (max_heap.empty() || num <= max_heap.top()) {
            max_heap.push(num);
        } else {
            min_heap.push(num);
        }

        // Balance the size of two heap
        if (max_heap.size() > min_heap.size() + 1) {
            min_heap.push(max_heap.top());
            max_heap.pop();
        } else if (min_heap.size() > max_heap.size()) {
            max_heap.push(min_heap.top());
            min_heap.pop();
        }
    }
    
    double findMedian() {
        if (max_heap.size() == min_heap.size()) {
            return (max_heap.top() + min_heap.top()) / 2.0;
        } else {
            return max_heap.top();
        }
    }
};
/**
 * Your MedianFinder object will be instantiated and called as such:
 * MedianFinder* obj = new MedianFinder();
 * obj->addNum(num);
 * double param_2 = obj->findMedian();
 */
```
