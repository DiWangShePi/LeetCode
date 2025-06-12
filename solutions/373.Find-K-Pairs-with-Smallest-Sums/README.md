# 373. Find K Pairs with Smallest Sums

### Description

ou are given two integer arrays nums1 and nums2 sorted in non-decreasing order and an integer k.

Define a pair (u, v) which consists of one element from the first array and one element from the second array.

Return the k pairs (u1, v1), (u2, v2), ..., (uk, vk) with the smallest sums.

### Example 

###### Example I

```
Input: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
Output: [[1,2],[1,4],[1,6]]
Explanation: The first 3 pairs are returned from the sequence: [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
```

###### Example II

```
Input: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
Output: [[1,1],[1,1]]
Explanation: The first 2 pairs are returned from the sequence: [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
```

### Solution

先说好想的解法：枚举所有可能的组合方式，计算和并放入到堆中，最终从堆里取出前k个即为答案。
考虑到我们仅需要前K个数据，我们也需要一直维护堆的大小为k。

```c++
class Node{
public:
    int x;
    int y;
    int sum;
    Node(int a, int b, int c): x(a), y(b), sum(c) {}
};

struct cmp {
    bool operator()(const Node &a, const Node &b){
       	return a.sum < b.sum;
    }
};

class Solution {
public:
    vector<vector<int>> kSmallestPairs(vector<int>& nums1, vector<int>& nums2, int k) {
        priority_queue<Node, vector<Node>, cmp> heap;
        for (int i = 0; i < k && i < nums1.size(); i++) {
            int num1 = nums1[i];
            for (int j = 0; j < k && j < nums2.size(); j++) {
                int num2 = nums2[j];
                Node current(num1, num2, num1 + num2);

                if (heap.size() == k) {
                    Node lar = heap.top();
                    if (lar.sum > current.sum) {
                        heap.pop();
                        heap.push(current);
                    }
                } else {
                    heap.push(current);
                }
                
            }
        }

        vector<vector<int>> an;
        for (int i = 0; i < k; i++) {
            Node current = heap.top();
            heap.pop();

            vector<int> re{current.x, current.y};
            an.push_back(re);
        }
        return an;
    }
};
```

或者我们也可以考虑逐步递增的方法，建立最小堆，每一次从堆中取出{i, j}，拓展{i + 1, j + 1}，重复K次。

```c++
class Solution {
public:
    vector<vector<int>> kSmallestPairs(vector<int>& nums1, vector<int>& nums2, int k) {
        vector<vector<int>> result;
        if (nums1.empty() || nums2.empty() || k == 0) return result;

        int m = nums1.size(), n = nums2.size();

        auto cmp = [](const tuple<int, int, int>& a, const tuple<int, int, int>& b) {
            return get<0>(a) > get<0>(b);
        };
        priority_queue<tuple<int, int, int>, vector<tuple<int, int, int>>, decltype(cmp)> minHeap(cmp);
        set<pair<int, int>> visited;

        minHeap.emplace(nums1[0] + nums2[0], 0, 0);
        visited.emplace(0, 0);

        while (!minHeap.empty() && result.size() < k) {
            auto [sum, i, j] = minHeap.top();
            minHeap.pop();
            result.push_back({nums1[i], nums2[j]});

            if (i + 1 < m && visited.emplace(i + 1, j).second) {
                minHeap.emplace(nums1[i + 1] + nums2[j], i + 1, j);
            }

            if (j + 1 < n && visited.emplace(i, j + 1).second) {
                minHeap.emplace(nums1[i] + nums2[j + 1], i, j + 1);
            }
        }

        return result;
    }
};
```