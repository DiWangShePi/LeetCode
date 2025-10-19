# 528. Random Pick with Weight

**Tags:** Binary Search, Prefix Sum, Weighted Random

### Description

You are given a 0-indexed array of positive integers w where w[i] describes the weight of the ith index.

You need to implement the function pickIndex(), which randomly picks an index in the range [0, w.length - 1] (inclusive) and returns it. The probability of picking an index i is w[i] / sum(w).

- For example, if w = [1, 3], the probability of picking index 0 is 1 / (1 + 3) = 0.25 (i.e., 25%), and the probability of picking index 1 is 3 / (1 + 3) = 0.75 (i.e., 75%).

### Example 

###### Example I

> Input
> ["Solution","pickIndex"]
> [[[1]],[]]
> Output
> [null,0]
> 
> Explanation
> Solution solution = new Solution([1]);
> solution.pickIndex(); // return 0. The only option is to return 0 since there is only one element in w.

###### Example II

> Input
> ["Solution","pickIndex","pickIndex","pickIndex","pickIndex","pickIndex"]
> [[[1,3]],[],[],[],[],[]]
> Output
> [null,1,1,1,1,0]
> 
> Explanation
> Solution solution = new Solution([1, 3]);
> solution.pickIndex(); // return 1. It is returning the second element (index = 1) that has a probability of 3/4.
> solution.pickIndex(); // return 1
> solution.pickIndex(); // return 1
> solution.pickIndex(); // return 1
> solution.pickIndex(); // return 0. It is returning the first element (index = 0) that has a probability of 1/4.
> 
> Since this is a randomization problem, multiple answers are allowed.
> All of the following outputs can be considered correct:
> [null,1,1,1,1,0]
> [null,1,1,1,1,1]
> [null,1,1,1,0,0]
> [null,1,1,1,0,1]
> [null,1,0,1,0,0]
> ......
> and so on.

### Solution

预先计算累加和和前缀和，每次在累加和范围内生成随机数，然后找哪个累加和第一个小于该随机数。

```c++
class Solution {
    vector<long long> prefix;
    long long sum;
    mt19937 gen;
    uniform_int_distribution<long long> dis;
public:
    Solution(vector<int>& w) : gen(random_device{}()) {
        prefix.resize(w.size());
        prefix[0] = w[0];
        for (int i = 1; i < w.size(); i++) {
            prefix[i] = prefix[i-1] + w[i];
        }
        sum = prefix.back();
        dis = uniform_int_distribution<long long>(0, sum - 1);
    }
    
    int pickIndex() {
        long long target = dis(gen);
        
        int left = 0, right = prefix.size() - 1;
        while (left < right) {
            int mid = left + (right - left) / 2;
            if (target < prefix[mid]) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }
};
/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(w);
 * int param_1 = obj->pickIndex();
 */
```
