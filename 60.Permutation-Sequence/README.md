# 60. Permutation Sequence

### 题目描述

给定集合[1,2....n]，共有n!中不同的排列方式。给定数字n和k，返回集合[1,2...n]的第k个排列。

**示例：**

```
Input: n = 3, k = 3
Output: "213"
```

```
Input: n = 4, k = 9
Output: "2314"
```

```
Input: n = 3, k = 1
Output: "123"
```

### 题目解析

### 代码实现

###### c++

```c++
class Solution {
public:
    string getPermutation(int n, int k) {
        vector<int> v={0};
        int tmp=1;
        for(int i=1;i<=n;i++){
            v.push_back(i);
            tmp*=i;
        }
        string s;
        cout<<tmp<<" ";
        for(int i=n;i>=2;i--){
            tmp/=i;
            int fl=(k+tmp-1)/tmp;
            s.push_back(v[fl]+'0');
            k-=(fl-1)*tmp;
            for(int j=fl;j<v.size()-1;j++){
                v[j]=v[j+1];
            }
        }
        s.push_back(v[1]+'0');   
        return s;
    }
};
```