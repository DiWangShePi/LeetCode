# 89. Gray Code

### Question Description

An n-bit gray code sequence is a sequence of 2n integers where:

- Every integer is in the inclusive range [0, 2n - 1],
- The first integer is 0,
- An integer appears no more than once in the sequence,
- The binary representation of every pair of adjacent integers differs by exactly one bit, and
- The binary representation of the first and last integers differs by exactly one bit.
Given an integer n, return any valid n-bit gray code sequence.

**Example: **

```
Input: n = 2
Output: [0,1,3,2]
Explanation:
The binary representation of [0,1,3,2] is [00,01,11,10].
- 00 and 01 differ by one bit
- 01 and 11 differ by one bit
- 11 and 10 differ by one bit
- 10 and 00 differ by one bit
[0,2,3,1] is also a valid gray code sequence, whose binary representation is [00,10,11,01].
- 00 and 10 differ by one bit
- 10 and 11 differ by one bit
- 11 and 01 differ by one bit
- 01 and 00 differ by one bit
```

```
Input: n = 1
Output: [0,1]
```

### Question Solution

利用现有的格雷码序列生成新的序列。初始时，格雷码序列包含 0 和 1。对于每增加一位，现有序列会被镜像，并在每个值的基础上加上当前位数的增量。具体步骤如下：

初始化格雷码序列为 [0, 1]。
对于从 1 到 n-1 的每一位，更新增量值 cnt 为 2 的当前位数次方。
通过对现有序列进行逆序遍历，将每个值加上 cnt 并添加到序列中。
返回生成的完整格雷码序列。
这样生成的格雷码具有相邻两个数仅有一位二进制位不同的性质，满足格雷码的定义。时间复杂度为 O(2^n)，空间复杂度为 O(2^n)，适用于生成较小的 n 值的格雷码。

### Code Implemption

###### c++

```c++
class Solution {
public:
    vector<int> grayCode(int n) {
        int cnt = 1;
        vector<int> v;
        v.push_back(0);
        v.push_back(1);
        for (int i = 1; i < n; i++){
            cnt *= 2;
            int l = v.size();
            for (int j = l - 1; j >= 0; j--){
                v.push_back(v[j] + cnt);
            }
        }
        return v;
    }
};
```