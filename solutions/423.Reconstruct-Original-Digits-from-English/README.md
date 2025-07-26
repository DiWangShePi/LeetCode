# 423. Reconstruct Original Digits from English

### Description

Given a string s containing an out-of-order English representation of digits 0-9, return the digits in ascending order.

### Example 

###### Example I

> Input: s = "owoztneoer"
> Output: "012"

###### Example II

> Input: s = "fviefuro"
> Output: "45"

### Solution

我一开始想的做法是，统计目标字符串中，所有字符出现的频率，随后从0到9，考虑每一个目标数字的字符串能在现有的字符串表中提取出多少个。

```c++
class Solution {
public:
    string originalDigits(string s) {
        vector<int> dict(26, 0);
        for (const char c : s) dict[c - 'a']++;

        vector<string> target = {"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
        string an = "";
        for (int i = 0; i < 10; i++) {
            int times = contains(dict, target[i]);
            if (times == 0) continue;
            for (int j = 0; j < times; j++) an = an + to_string(i);
        }
        return an;
    }

private:
    int contains(vector<int>& dict, string t) {
        // 统计目标字符串中的字符出现次数
        vector<int> t_dict(26, 0);
        for (char c : t) t_dict[c - 'a']++;

        // 确认现在的dict中，还存在能构成至少一个目标字符串的字符
        for (int i = 0; i < t_dict.size(); i++) {
            if (t_dict[i] != 0 && dict[i] < t_dict[i]) return 0; 
        }

        // 检查现有的字符中，最多能构成几个目标字符
        int max_times = INT_MAX;
        for (int i = 0; i < t_dict.size(); i++) {
            if (t_dict[i] == 0) continue;
            max_times = min(max_times, dict[i] / t_dict[i]);
        }

        // 更新现有的字符
        for (int i = 0; i < t_dict.size(); i++) {
            if (t_dict[i] == 0) continue;
            dict[i] -= max_times * t_dict[i];
        }
        return max_times;
    }
};
```

但这种贪心的解法存在一定问题，用于构成后续数字的部分字符，可能在前面被其他数字占用了，最终导致部分数字无法出现，部分字符没用完的问题。

比如如下案例：

> "zeroonetwothreefourfivesixseveneightnine"

我们应该给出如下答案

> 0123456789

但构成1的字符会占据掉其他数字的必要组成字符，最后给出如下答案

> 01113356

因此，我们需要换一个解法。注意到给出的字符串是合法的，因此我们可以用数字中的特殊字符来唯一的确定某一个数字出现的次数。

比如z, w, u, x, g 都只在一个数字中，即 0,2,4,6,8 出现，那么统计这些字符的出现次数，就可以确定相应数字应该出现的次数。

```c++
class Solution {
public:
    string originalDigits(string s) {
        unordered_map<int, int> dict;
        for (char c: s) dict[c]++;

        vector<int> count(10);
        count[0] = dict['z'];
        count[2] = dict['w'];
        count[4] = dict['u'];
        count[6] = dict['x'];
        count[8] = dict['g'];

        count[3] = dict['h'] - count[8];
        count[5] = dict['f'] - count[4];
        count[7] = dict['s'] - count[6];

        count[1] = dict['o'] - count[0] - count[2] - count[4];

        count[9] = dict['i'] - count[5] - count[6] - count[8];

        string ans;
        for (int i = 0; i < 10; ++i) {
            for (int j = 0; j < count[i]; ++j) {
                ans += char(i + '0');
            }
        }
        return ans;
    }
};
```
