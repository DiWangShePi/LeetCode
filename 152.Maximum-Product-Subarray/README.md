# 152. Maximum Product Subarray

### Description

Given an integer array nums, find a subarray that has the largest product, and return the product.

The test cases are generated so that the answer will fit in a 32-bit integer.

### Solution

如果我们用 $f_{\text{max}}(i)$ 表示以第 $i$ 个元素结尾的乘积最大的子数组的乘积，$a$ 表示输入参数 $nums$，那么根据「53. 最大子序和」的经验，我们很容易得出这样的状态转移方程：

$$
f_{\text{max}}(i) = \max \{ f_{\text{max}}(i-1) \times a_i, a_i \}
$$

它表示以第 $i$ 个元素结尾的乘积最大的子数组的乘积可以考虑 $a_i$ 加入前面的 $f_{\text{max}}(i-1)$ 对应的一段，或者单独成为一段，这里两种情况取最大值。求出所有 $f_{\text{max}}(i)$ 之后选取最大的一个作为答案。

###### 可是这里，这样做是错误的。为什么呢？

因为这里的定义并不满足「最优子结构」。具体地讲，如果 $a = \{5, 6, -3, 4, -3\}$，那么此时 $f_{\text{max}}$ 对应的序列是 $\{5, 30, -4, -3\}$，按照前面的算法我们可以得到答案为 $30$，即前两个数的乘积，而实际答案应该是全体数字的乘积。我们想一想问题出在哪里呢？问题出在最后一个 $-3$ 所对应的 $f_{\text{max}}$ 的值既不是 $-3$，也不是 $4 \times (-3)$，而是：

$$
5 \times 6 \times (-3) \times 4 \times (-3)
$$

所以我们得到了一个结论：当前位置的最优解未必是由前一个位置最优解转移得到的。

###### 我们可以根据正负性进行分类讨论

考虑当前位置如果是一个负数，那么我们希望以它为结尾的某个位置结果的某个段的积也是负数，这样就可以负得负，且我们希望这个积尽可能「负得更多」，即尽可能小。如果当前的位置是一个正数，那么我们更希望以它前一个位置结果的某个段的积也是正数，并且希望它尽可能地大。于是这里我们可以再维护一个 $f_{\text{min}}(i)$，它表示以第 $i$ 个元素结尾的最小子数组的乘积，并且希望这个积尽可能地小。于是我们的状态转移方程变为：

$$
f_{\text{max}}(i) = \max \{ f_{\text{max}}(i-1) \times a_i, f_{\text{min}}(i-1) \times a_i, a_i \}
$$

$$
f_{\text{min}}(i) = \min \{ f_{\text{max}}(i-1) \times a_i, f_{\text{min}}(i-1) \times a_i, a_i \}
$$

它代表第 $i$ 元素结尾的子数组中乘积最大子数组的乘积 $f_{\text{max}}(i)$，可以考虑把 $a_i$ 加入前一个元素结尾的子数组的乘积中，二者相加，三者取大；而对于第 $i$ 个元素结尾的子数组中最小子数组的乘积 $f_{\text{min}}(i)$，同理。



### Implementation

###### c++

```c++
class Solution {
public:
    int maxProduct(vector<int>& nums) {
        vector <long> maxF(nums.begin(),nums.end()), minF(nums.begin(), nums.end());
        for (int i = 1; i < nums.size(); ++i) {
            maxF[i] = max(maxF[i - 1] * nums[i], max((long)nums[i], minF[i - 1] * nums[i]));
            minF[i] = min(minF[i - 1] * nums[i], min((long)nums[i], maxF[i - 1] * nums[i]));
        }
        return *max_element(maxF.begin(), maxF.end());
    }
};
```
