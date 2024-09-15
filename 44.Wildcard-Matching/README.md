# 44. Wildcard Matching

### 题目描述

给定一个输入字符串 (s) 和一个模式 (p)，实现支持“?”和“*”的通配符模式匹配，其中：

“?”匹配任何单个字符。
“*”匹配任何字符序列（包括空序列）。
匹配应覆盖整个输入字符串（而不是部分）。

**示例：**

```
Input: s = "aa", p = "a"
Output: false
Explanation: "a" does not match the entire string "aa".
```

```
Input: s = "aa", p = "*"
Output: true
Explanation: '*' matches any sequence.
```

```
Input: s = "cb", p = "?a"
Output: false
Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.
```

### 题目解析

递归的解决这个问题，从后往前遍历两个字符串s和p。

若两个字符串当前比较的字符相等，即为合法，可进行至下一位，将指向两个字符串的相应指针都减1。
若此时p字符串的字符为?，则s字符串可以为任何字符，同样合法，可进行至下一位，将指向两个字符串的相应指针都减1。
若此时p字符串的字符为*，则需要考虑这个*还可以匹配尚未遇到的s中的字符，可以将其保留。也有可能前面的q和s同样可以匹配。
因此此时的匹配结果取决于s[0:current]与p[0:current-1]是否匹配，和s[0:current-1]和p[0:current]是否匹配。
两者有一个匹配即可。

此外，当两者长度都为0时，即匹配完成，返回true。
当s的长度为0，但p尚未完成时，p剩下的字符需要都是*，否则即为false。
当s的长度不为0，但p为0时，即为false。

### 代码实现

###### c++

```c++
class Solution {
private:
    bool solveSO(string s,string p)
    {
        int n=s.length();
        int m=p.length();
        vector<int> prev(m+1,0);
        vector<int> curr(m+1,0);
        
        prev[0]=true;

        for(int j=1;j<=m;j++)
        {
            bool flag=true;
            for(int k=1;k<=j;k++)
            {
                if(p[k-1]!='*')
                {
                    flag=false;
                    break;
                }
            }
            prev[j]=flag;
        }

        for(int i=1;i<=n;i++)
        {
            for(int j=1;j<=m;j++)
            {
                //match
                if(s[i-1]==p[j-1] || p[j-1]=='?')  curr[j]=prev[j-1];
                else if(p[j-1]=='*') 
                     curr[j]= ( prev[j]|| curr[j-1]);
                else curr[j]=false;
            }
            prev=curr;
        }

        return prev[m];
    }     
public:
    bool isMatch(string s, string p) {
        return solveSO(s,p);
    }
};
```