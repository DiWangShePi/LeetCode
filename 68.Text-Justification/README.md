# 68. Text Justification

### 题目描述

给定一个字符串数组 words 和一个宽度 maxWidth，格式化文本，使每行恰好有 maxWidth 个字符并且完全（左和右）对齐。

您应该以贪婪的方式打包您的单词；也就是说，在每行中打包尽可能多的单词。必要时填充额外的空格 ' '，以便每行恰好有 maxWidth 个字符。

单词之间的额外空格应尽可能均匀分布。如果一行上的空格数不能均匀地划分在单词之间，则左侧的空位将比右侧的空位分配更多的空格。

对于最后一行文本，它应该是左对齐的，并且单词之间不插入额外的空格。

**示例：**

```
Input: words = ["This", "is", "an", "example", "of", "text", "justification."], maxWidth = 16
Output:
[
   "This    is    an",
   "example  of text",
   "justification.  "
]
```

```
Input: words = ["What","must","be","acknowledgment","shall","be"], maxWidth = 16
Output:
[
  "What   must   be",
  "acknowledgment  ",
  "shall be        "
]
Explanation: Note that the last line is "shall be    " instead of "shall     be", because the last line must be left-justified instead of fully-justified.
Note that the second line is also left-justified because it contains only one word.
```

```
Input: words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"], maxWidth = 20
Output:
[
  "Science  is  what we",
  "understand      well",
  "enough to explain to",
  "a  computer.  Art is",
  "everything  else  we",
  "do                  "
]
```

### 题目解析

初始化一个空的数组，遍历输入的数组，对于每一个新的字符串，检查这个字符串的加入是否会使得当前保留数组中所有元素长度之和超过上界。
- 若不会，则将当前元素加入到数组中。
- 若会，则将已有数组按规则加入到结果中，更新保留数组为加入当前元素。
遍历结束后将保留数组中的元素按最后一行的要求添加到结尾。

### 代码实现

###### c++

my solution
```c++
class Solution {
public:
    vector<string> fullJustify(vector<string>& words, int maxWidth) {
        vector<string> result;
        vector<string> stack;
        int wordsSum = 0, interval = -1;
        for (int i = 0; i < words.size(); i++) {
            string current = words[i];
            if (wordsSum + current.length() > maxWidth) {
                int intervalLength = maxWidth - wordsSum;
                int intervalSize = interval != 0 ? (intervalLength + interval - 1) / interval : 0;
                string currentResult = stack[0];
                for (int j = 1; j < stack.size(); j++) {
                    if (j == stack.size() - 1) { // update interval size for the last word
                        intervalSize = maxWidth - currentResult.length() - stack[stack.size() - 1].length();
                    }

                    for (int k = 0; k < intervalSize; k++) {
                        currentResult = currentResult + " ";
                    }
                    currentResult = currentResult + stack[j];
                }
                if (currentResult.length() < maxWidth) {
                    for (int k = 0; k < maxWidth - currentResult.length(); k++) {
                        currentResult = currentResult + " ";
                    }
                }
                
                result.push_back(currentResult);
                stack.clear();
                wordsSum = 0;
                interval = -1;
            }

            stack.push_back(current);
            wordsSum += current.length();
            interval += 1;
        }

        // handles the last few words in stack
        string currentResult = "";
        for (int i = 0; i < stack.size() - 1; i++) {
            currentResult = currentResult + stack[i];
            currentResult = currentResult + " ";
        }
        currentResult = currentResult + stack[stack.size() - 1];
        for (int i = 0; i < maxWidth - currentResult.length(); i++) {
            currentResult = currentResult + " ";
        }
        result.push_back(currentResult);

        return result;
    }
};
```

others solution
```c++
class Solution {
public:
    vector<string> fullJustify(vector<string>& words, int maxWidth) {
        vector<string> result;
        int start = 0, totSize = 0, wordCount = 0;

        for (int i = 0; i < words.size(); ++i) {
            int wordSize = words[i].size();
            wordCount++;

            // Check if current line fits within maxWidth
            if (totSize + wordSize + (wordCount - 1) <= maxWidth) {
                totSize += wordSize;

                // Handle last line case
                if (i == words.size() - 1) {
                    string lastLine;
                    for (int j = start; j <= i; ++j) {
                        lastLine += words[j];
                        if (j != i) lastLine += " ";
                    }
                    while (lastLine.size() < maxWidth) lastLine += " ";
                    result.push_back(lastLine);
                    break;
                }
            } else {
                // If only one word fits, left justify it
                if (wordCount - 1 == 1) {
                    string singleWordLine = words[i - 1];
                    while (singleWordLine.size() < maxWidth) singleWordLine += " ";
                    result.push_back(singleWordLine);

                    // Reset for next line
                    totSize = wordSize;
                    start = i;
                    wordCount = 1;

                    // Handle last word
                    if (i == words.size() - 1) {
                        string lastWord = words[i];
                        while (lastWord.size() < maxWidth) lastWord += " ";
                        result.push_back(lastWord);
                    }
                } else {
                    // Justify the line
                    int spaces = maxWidth - totSize;
                    string justifiedLine;
                    for (int j = start; j < i; ++j) {
                        justifiedLine += words[j];
                        if (j < i - 1) {
                            int spaceToAdd = spaces / (wordCount - 2);
                            if (spaces % (wordCount - 2) != 0) spaceToAdd++;
                            justifiedLine += string(spaceToAdd, ' ');
                            spaces -= spaceToAdd;
                            wordCount--;
                        }
                    }
                    result.push_back(justifiedLine);

                    // Reset for next line
                    totSize = wordSize;
                    start = i;
                    wordCount = 1;

                    // Handle last word
                    if (i == words.size() - 1) {
                        string lastWord = words[i];
                        while (lastWord.size() < maxWidth) lastWord += " ";
                        result.push_back(lastWord);
                    }
                }
            }
        }
        return result;
    }
};
```