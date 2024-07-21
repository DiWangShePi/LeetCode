# 6. Zigzag Conversion

### 题目描述

给定一个字符串，和一个整数n，将它排列成一个n行的蛇形返回。

**示例:**

```
P   A   H   N
A P L S I I G
Y   I   R
And then read line by line: "PAHNAPLSIIGYIR"

Write the code that will take a string and make this conversion given a number of rows:

string convert(string s, int numRows);
Example 1:

Input: s = "PAYPALISHIRING", numRows = 3
Output: "PAHNAPLSIIGYIR"
Example 2:

Input: s = "PAYPALISHIRING", numRows = 4
Output: "PINALSIGYAHRPI"
Explanation:

P     I    N
A   L S  I G
Y A   H R
P     I
```

给定字符串和蛇形排列占据的行数，要求返回重新排列后的串

### 题目解析

这道题是一道模拟题，题目的要求就是答案，我们只需要读懂题意就很容易实现。

我们最终要输出的是以蛇形摆放之后的字符串再按行串联在一起之后的结果，也就是说每一个字母摆放的列并不重要，重要的是摆放的行号。我们可以很容易想到通过数组维护每一行当中摆放的字母，最后将每一行的结果串联即可。所以问题就只剩下了，我们如何知道每一个字母应该摆放在哪一行？

其实这也是有规律的，我们通过观察样例可以发现，我们每一个字母摆放的行号先是从0递增到n-1，再从n-1递减到0。我们就模拟这个过程，一个字符一个字符的放置即可。

比如字符串是“PAYPALISHIRING ”，rowNum=4。我们可以创建四个空串：

“”
“”
“”
“”

然后我们按照蛇形一个字母一个字母地放进这些空串当中：

当放了第一个字母p之后，变成：

“p”
“”
“”
“”

接着放第二个：

“p”
“a”
“”
“”

接着第三个：

“p”
“a”
“y”
“”

当我们把所有字母都放完了之后，可以得到这样的四个串：

“PIN”
“ALSIG”
“YAHR”
“PI”

然后把这四串拼接在一起就行了。

### 代码实现

###### c++
```c++
class Solution {
public:
    string convert(string s, int numRows) {
        if (numRows == 1) {
            return s;
        }

        std::vector<string> array(numRows, "");
        for(int i = 0; i < s.length(); i++) {
            char currentChar = s[i];
            int position = i % (2 * numRows - 2);

            if (position < numRows) {
                array[position] += currentChar;
            } else {
                int target = 2 * numRows - 2 - position;
                for(int j = 0; j < numRows; j++) {
                    if (j == target) {
                        array[j] += currentChar;
                    } else {
                        array[j] += ' ';
                    }
                }
            }
        }

        std::string answer = s.substr(0, 0);
        for(int i = 0; i < numRows; i++) {
            std::string currentString = array[i];
            cout << currentString << endl;

            currentString.erase(std::remove(currentString.begin(), currentString.end(), ' '), currentString.end());
            answer += currentString;
        }
        return answer;
    }
};
```