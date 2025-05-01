# 301. Remove Invalid Parentheses

### Description

Given a string s that contains parentheses and letters, remove the minimum number of invalid parentheses to make the input string valid.

Return a list of unique strings that are valid with the minimum number of removals. You may return the answer in any order.

### Example 

###### Example I

```
Input: s = "()())()"
Output: ["(())()","()()()"]
```

###### Example II

```
Input: s = "(a)())()"
Output: ["(a())()","(a)()()"]
```

###### Example III

```
Input: s = ")("
Output: [""]
```

### Solution

> 就是暴力回溯，枚举所有可能性，懒得写细节了，于是借用一下官方的题解

题目让我们删除括号使得剩下的括号匹配，要求我们删除最少的括号数，并且要求得到所有的结果。我们可以使用回溯算法，尝试遍历所有可能的去掉非法括号的方案。

首先我们利用括号匹配的规则求出该字符串 s 中最少需要去掉的左括号的数目 lremove 和右括号的数目 rremove，然后我们尝试在原字符串 s 中去掉 lremove 个左括号和 rremove 个右括号，然后检测剩余的字符串是否合法匹配，如果合法匹配则我们则认为该字符串为可能的结果，我们利用回溯算法来尝试搜索所有可能的去除括号的方案。

在进行回溯时可以利用以下的剪枝技巧来增加搜索的效率：

我们从字符串中每去掉一个括号，则更新 lremove 或者 rremove，当我们发现剩余未尝试的字符串的长度小于 lremove+rremove 时，则停止本次搜索。
当 lremove 和 rremove 同时为 0 时，则我们检测当前的字符串是否合法匹配，如果合法匹配则我们将其记录下来。
由于记录的字符串可能存在重复，因此需要对重复的结果进行去重，去重的办法有如下两种：

利用哈希表对最终生成的字符串去重。
我们在每次进行搜索时，如果遇到连续相同的括号我们只需要搜索一次即可，比如当前遇到的字符串为 "(((())"，去掉前四个左括号中的任意一个，生成的字符串是一样的，均为 "((())"，因此我们在尝试搜索时，只需去掉一个左括号进行下一轮搜索，不需要将前四个左括号都尝试一遍。

```c++
class Solution {
public:
    vector<string> an;

    vector<string> removeInvalidParentheses(string s) {
        int lremove = 0, rremove = 0;
        for (int i = 0; i < s.size(); i++) {
            if (s[i] == '(') lremove++;
            else if (s[i] == ')')
            {
                if (lremove == 0) rremove++;
                else lremove--;
            }
        }

        helper(s, 0, lremove, rremove);
        return an;
    }

private:
    void helper(string str, int start, int lremove, int rremove) {
        if (lremove == 0 && rremove == 0) 
        {
            if (isValid(str))
            {
                an.push_back(str);
                return;
            }
        }

        for (int i = start; i < str.size(); i++) 
        {
            if (i != start && str[i] == str[i - 1]) continue;

            if (lremove + rremove > str.size() - i) return;
            if (lremove > 0 && str[i] == '(') helper(str.substr(0, i) + str.substr(i + 1), i, lremove - 1, rremove);
            if (rremove > 0 && str[i] == ')') helper(str.substr(0, i) + str.substr(i + 1), i, lremove, rremove - 1);
        }
    }

    inline bool isValid(const string & str) {
        cout << str << endl;

        int cnt = 0;
        for (int i = 0; i < str.size(); i++) 
        {
            if (str[i] == '(') cnt++;
            else if (str[i] == ')')
            {
                cnt--;
                if (cnt < 0) return false;
            }
        }
        return cnt == 0;
    }
};
```
