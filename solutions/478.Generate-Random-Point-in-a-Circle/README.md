# 478. Generate Random Point in a Circle

### Description

Given the radius and the position of the center of a circle, implement the function randPoint which generates a uniform random point inside the circle.

Implement the Solution class:

- Solution(double radius, double x_center, double y_center) initializes the object with the radius of the circle radius and the position of the center (x_center, y_center).
- randPoint() returns a random point inside the circle. A point on the circumference of the circle is considered to be in the circle. The answer is returned as an array [x, y].

### Example 

###### Example I

> Input
> ["Solution", "randPoint", "randPoint", "randPoint"]
> [[1.0, 0.0, 0.0], [], [], []]
> Output
> [null, [-0.02493, -0.38077], [0.82314, 0.38945], [0.36572, 0.17248]]
> 
> Explanation
> Solution solution = new Solution(1.0, 0.0, 0.0);
> solution.randPoint(); // return [-0.02493, -0.38077]
> solution.randPoint(); // return [0.82314, 0.38945]
> solution.randPoint(); // return [0.36572, 0.17248]

### Solution

拒绝采样是指，我们在可以保证的范围内随机的生成点，当生成的点不符合我们的要求时，将其“拒绝”。

```c++
class Solution {
private:
    double x_center, y_center, radius;
    
public:
    Solution(double radius, double x_center, double y_center) {
        this->radius = radius;
        this->x_center = x_center;
        this->y_center = y_center;
        srand(time(0)); 
    }
    
    vector<double> randPoint() {
        while (true) {
            double x = x_center - radius + 2 * radius * ((double)rand() / RAND_MAX);
            double y = y_center - radius + 2 * radius * ((double)rand() / RAND_MAX);
            
            if (pow(x - x_center, 2) + pow(y - y_center, 2) <= pow(radius, 2)) {
                return {x, y};
            }
        }
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(radius, x_center, y_center);
 * vector<double> param_1 = obj->randPoint();
 */
```

另一个有趣的解法是在[0, r)的范围内随机生成边长，在[0, 2pi]的范围内随机生成角度，然后计算坐标返回。
但这并不是合理的解法，因为靠近圆心的点会有比远离圆心的更大的选中概率。
这相当于一条边在圆内均匀的旋转（因为生成的角度是随机的），此时不同边长的线速度不是一样的，那么他们在空间中出现的概率也就不是一样的。
