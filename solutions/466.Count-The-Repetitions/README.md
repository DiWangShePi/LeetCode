# 466. Count The Repetitions

**Tags:** String, Dynamic Programming

### Description

We define str = [s, n] as the string str which consists of the string s concatenated n times.

For example, str == ["abc", 3] =="abcabcabc".
We define that string s1 can be obtained from string s2 if we can remove some characters from s2 such that it becomes s1.

For example, s1 = "abc" can be obtained from s2 = "abdbec" based on our definition by removing the bolded underlined characters.
You are given two strings s1 and s2 and two integers n1 and n2. You have the two strings str1 = [s1, n1] and str2 = [s2, n2].

Return the maximum integer m such that str = [str2, m] can be obtained from str1.

### Example 

###### Example I

> Input: s1 = "acb", n1 = 4, s2 = "ab", n2 = 2
> Output: 2

###### Example II

> Input: s1 = "acb", n1 = 1, s2 = "acb", n2 = 1
> Output: 1

### Solution

我们先给出一个暴力的解法：遍历s1，匹配s2，用能匹配到的数字除以n2即为结果。

```c++
class Solution {
public:
    int getMaxRepetitions(string s1, int n1, string s2, int n2) {
        int an = 0, index = 0;
        int n = s1.size(), m = s2.size();
        while (true) {
            for (char c : s1) {
                if (c == s2[index]) {
                    index++;
                    if (index == m) {
                        index = 0;
                        an++;
                    }
                }
            }
            
            n1 -= 1;
            if (n1 == 0) break;
        }
        return an / n2;
    }
};
```

但我们随后发现，由于s2也是重复的，我们就可以记录下s2在一次匹配后出现的位置。如果再次遇到，我们就知道这中间，s1完整的出现了几次，s2也完整的出现了几次。随后我们可以通过数学计算简便中间的重复过程。

```c++
class Solution {
public:
    int getMaxRepetitions(string s1, int n1, string s2, int n2) {
        if (n1 == 0) return 0;

        int s1cnt = 0, s2cnt = 0, index = 0;
        unordered_map<int, pair<int,int>> recall;

        while (true) {
            s1cnt++;
            for (char c : s1) {
                if (c == s2[index]) {
                    index++;
                    if (index == s2.size()) {
                        index = 0;
                        s2cnt++;
                    }
                }
            }

            if (s1cnt == n1) {
                return s2cnt / n2;
            }

            if (recall.count(index)) {
                auto [pre_s1cnt, pre_s2cnt] = recall[index];
                int pre_loop = pre_s1cnt;
                int in_loop_s1 = s1cnt - pre_s1cnt;
                int in_loop_s2 = s2cnt - pre_s2cnt;

                int ans = pre_s2cnt;
                ans += (n1 - pre_loop) / in_loop_s1 * in_loop_s2;
                int rest = (n1 - pre_loop) % in_loop_s1;
                for (int i = 0; i < rest; i++) {
                    for (char c : s1) {
                        if (c == s2[index]) {
                            index++;
                            if (index == s2.size()) {
                                index = 0;
                                ans++;
                            }
                        }
                    }
                }
                return ans / n2;
            } else {
                recall[index] = {s1cnt, s2cnt};
            }
        }
    }
};
```