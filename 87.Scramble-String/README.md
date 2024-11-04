# 87. Scramble String

### Question Description

We can scramble a string s to get a string t using the following algorithm:

If the length of the string is 1, stop.
If the length of the string is > 1, do the following:
Split the string into two non-empty substrings at a random index, i.e., if the string is s, divide it to x and y where s = x + y.
Randomly decide to swap the two substrings or to keep them in the same order. i.e., after this step, s may become s = x + y or s = y + x.
Apply step 1 recursively on each of the two substrings x and y.
Given two strings s1 and s2 of the same length, return true if s2 is a scrambled string of s1, otherwise, return false.

**Example: **

```
Input: s1 = "great", s2 = "rgeat"
Output: true
Explanation: One possible scenario applied on s1 is:
"great" --> "gr/eat" // divide at random index.
"gr/eat" --> "gr/eat" // random decision is not to swap the two substrings and keep them in order.
"gr/eat" --> "g/r / e/at" // apply the same algorithm recursively on both substrings. divide at random index each of them.
"g/r / e/at" --> "r/g / e/at" // random decision was to swap the first substring and to keep the second substring in the same order.
"r/g / e/at" --> "r/g / e/ a/t" // again apply the algorithm recursively, divide "at" to "a/t".
"r/g / e/ a/t" --> "r/g / e/ a/t" // random decision is to keep both substrings in the same order.
The algorithm stops now, and the result string is "rgeat" which is s2.
As one possible scenario led s1 to be scrambled to s2, we return true.
```

```
Input: s1 = "abcde", s2 = "caebd"
Output: false
```

```
Input: s1 = "a", s2 = "a"
Output: true
```

### Solution

> 参考力扣中文官方题解

### Code Implemption

###### c++

```c++
class Solution {
private:
    // 记忆化搜索存储状态的数组
    // -1 表示 false，1 表示 true，0 表示未计算
    int memo[30][30][31];
    string s1, s2;

public:
    bool checkIfSimilar(int i1, int i2, int length) {
        unordered_map<int, int> freq;
        for (int i = i1; i < i1 + length; ++i) {
            ++freq[s1[i]];
        }
        for (int i = i2; i < i2 + length; ++i) {
            --freq[s2[i]];
        }
        if (any_of(freq.begin(), freq.end(), [](const auto& entry) {return entry.second != 0;})) {
            return false;
        }
        return true;
    }

    // 第一个字符串从 i1 开始，第二个字符串从 i2 开始，子串的长度为 length，是否和谐
    bool dfs(int i1, int i2, int length) {
        if (memo[i1][i2][length]) {
            return memo[i1][i2][length] == 1;
        }

        // 判断两个子串是否相等
        if (s1.substr(i1, length) == s2.substr(i2, length)) {
            memo[i1][i2][length] = 1;
            return true;
        }

        // 判断是否存在字符 c 在两个子串中出现的次数不同
        if (!checkIfSimilar(i1, i2, length)) {
            memo[i1][i2][length] = -1;
            return false;
        }
        
        // 枚举分割位置
        for (int i = 1; i < length; ++i) {
            // 不交换的情况
            if (dfs(i1, i2, i) && dfs(i1 + i, i2 + i, length - i)) {
                memo[i1][i2][length] = 1;
                return true;
            }
            // 交换的情况
            if (dfs(i1, i2 + length - i, i) && dfs(i1 + i, i2, length - i)) {
                memo[i1][i2][length] = 1;
                return true;
            }
        }

        memo[i1][i2][length] = -1;
        return false;
    }

    bool isScramble(string s1, string s2) {
        memset(memo, 0, sizeof(memo));
        this->s1 = s1;
        this->s2 = s2;
        return dfs(0, 0, s1.size());
    }
};
```

If you are interested in a more effective example:

```c++
class Solution {
public:
    bool isScramble(string s1, string s2) {
        const int sz = s1.size();
        int8_t memo[sz][sz][sz+1];
        memset(memo,-1,sizeof(memo));

        function<bool(int,int,int)>dfs= [&](int idx1, int idx2, int len)->bool{
            if(memo[idx1][idx2][len]!=-1)return (memo[idx1][idx2][len]==1);
            
            if(len==1&&s1[idx1]==s2[idx2])return true;
            
            int count[26];
            memset(count,0,sizeof(count));

            for(int i = 0;i<len;i++){
                count[s1[idx1+i]-'a']++;
                count[s2[idx2+i]-'a']--;
            }
            for(int i = 0;i<26;i++){
                if(count[i]!=0){
                    memo[idx1][idx2][len] = 0;
                    return false;
                }
            }

            for(int i = 1;i<len;i++){
                if ((dfs(idx1, idx2, i) && dfs(idx1 + i, idx2 + i, len - i)) ||
                    (dfs(idx1, idx2 + len - i, i) && dfs(idx1 + i, idx2, len - i))) {
                    
                    memo[idx1][idx2][len] = 1;
                    return true;
                }
            }

            memo[idx1][idx2][len] = 0;
            return false;
        };

        return dfs(0,0,sz);
    }
};
```