# 497. Random Point in Non-overlapping Rectangles

### Description

You are given an array of non-overlapping axis-aligned rectangles rects where rects[i] = [ai, bi, xi, yi] indicates that (ai, bi) is the bottom-left corner point of the ith rectangle and (xi, yi) is the top-right corner point of the ith rectangle. Design an algorithm to pick a random integer point inside the space covered by one of the given rectangles. A point on the perimeter of a rectangle is included in the space covered by the rectangle.

Any integer point inside the space covered by one of the given rectangles should be equally likely to be returned.

Note that an integer point is a point that has integer coordinates.

Implement the Solution class:

- Solution(int[][] rects) Initializes the object with the given rectangles rects.
- int[] pick() Returns a random integer point [u, v] inside the space covered by one of the given rectangles.

### Example 

###### Example I

![](./lc-pickrandomrec.jpg)

> Input
> ["Solution", "pick", "pick", "pick", "pick", "pick"]
> [[[[-2, -2, 1, 1], [2, 2, 4, 6]]], [], [], [], [], []]
> Output
> [null, [1, -2], [1, -1], [-1, -2], [-2, -2], [0, 0]]
> 
> Explanation
> Solution solution = new Solution([[-2, -2, 1, 1], [2, 2, 4, 6]]);
> solution.pick(); // return [1, -2]
> solution.pick(); // return [1, -1]
> solution.pick(); // return [-1, -2]
> solution.pick(); // return [-2, -2]
> solution.pick(); // return [0, 0]

### Solution

初始化的时候计算矩形面积，然后根据面积作为权重随机选择一个矩形，然后再在该矩形内随机生成点。

```c++
class Solution {
public:
    vector<double> probabilities; 
    vector<vector<int>> local_rects;
    mt19937 gen; 
    uniform_real_distribution<double> prob_dis;  

    Solution(vector<vector<int>>& rects) : gen(random_device{}()), prob_dis(0.0, 1.0) {
        vector<long long> areas;
        long long total_area = 0;
        
        for (vector<int>& r : rects) {
            long long area = (long long)(r[2] - r[0] + 1) * (r[3] - r[1] + 1);
            areas.push_back(area);
            total_area += area;
        }
        
        probabilities.resize(rects.size());
        double cumulative = 0.0;
        for (int i = 0; i < rects.size(); i++) {
            cumulative += (double)areas[i] / total_area;
            probabilities[i] = cumulative;
        }
        
        local_rects = rects;
    }
    
    vector<int> pick() {
        double rand_prob = prob_dis(gen);
        int selected_idx = 0;
        
        for (int i = 0; i < probabilities.size(); i++) {
            if (rand_prob <= probabilities[i]) {
                selected_idx = i;
                break;
            }
        }
        
        vector<int>& rect = local_rects[selected_idx];
        uniform_int_distribution<int> x_dis(rect[0], rect[2]);
        uniform_int_distribution<int> y_dis(rect[1], rect[3]);
        
        return {x_dis(gen), y_dis(gen)};
    }
};
```
