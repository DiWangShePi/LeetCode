# 30. Substring with Concatenation of All Words

### 题目描述

给定字符串s和一个字符串列表words，words中的所有字符串长度一致。

一个连接的字符串是一个包含了words中所有单词的字符串，这些单词可以以任意顺序排列。

返回s中所有连接子字符串的起始索引的数组。你可以以任何顺序返回答案。

**示例：**

```
Input: s = "barfoothefoobarman", words = ["foo","bar"]

Output: [0,9]

Explanation:

The substring starting at 0 is "barfoo". It is the concatenation of ["bar","foo"] which is a permutation of words.
The substring starting at 9 is "foobar". It is the concatenation of ["foo","bar"] which is a permutation of words.
```

```
Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]

Output: []

Explanation:

There is no concatenated substring.
```

```
Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]

Output: [6,9,12]

Explanation:

The substring starting at 6 is "foobarthe". It is the concatenation of ["foo","bar","the"].
The substring starting at 9 is "barthefoo". It is the concatenation of ["bar","the","foo"].
The substring starting at 12 is "thefoobar". It is the concatenation of ["the","foo","bar"].
```

### 题目解析

首先，我们遍历words中的所有单词，并将结果记录在一个字典中。

在上一步中，我们随机选择其中一个单词，并将其所处的所有位置添加到答案的候选人列表中。

考虑到所有的单词长度一致，我们接下来遍历候选人列表。每一轮中，寻找字典中是否存在上界和下界，并检查该单词是否已经出现在该候选人的字符串中。若候选人找不到上界也找不到下界，则从候选人中删除该答案。当候选人的长度已经与答案一致，即为实际答案。当候选人列表不再更新时，循环结束。

### 代码实现

```c++
class Solution {
    std::unordered_map<std::string, unsigned int> map;
public:
    vector<int> findSubstring(string s, vector<string>& words) {
        std::vector<int> result;
        unsigned int length = words[0].size();

        map.clear();
        for (const std::string& word : words)
            map[word]++;

        for (unsigned int offset = 0; offset < length; ++offset) {
            unsigned int size = 0;
            std::unordered_map<std::string, unsigned int> seen;
            for (unsigned int i = offset; i + length <= s.size(); i += length) {
                std::string sub = s.substr(i, length);

                auto itr = map.find(sub);
                if (itr == map.end()) {
                    seen.clear();
                    size = 0;
                    continue;
                }

                ++seen[sub];
                ++size;
                while (seen[sub] > itr->second) {
                    std::string first = s.substr(i - (size - 1) * length, length);
                    --seen[first];
                    --size;
                }
                
                if (size == words.size())
                    result.push_back(i - (size - 1) * length);
            }
        }

        return result;
    }
};
```

Solution using dfs (Time limit exceeded)

```c++
#include<map>
#include<string>
#include<vector>
#include<iostream>
#include <set>

using namespace std;

class Solution {
public:
    vector<int> findSubstring(string s, vector<string>& words) {
        map<string, int> wordMap;
        for (const auto& word : words) {
            wordMap[word] += 1;
        }

        vector<int> candidateList(s.length()+1, 0);
        for (auto & iter : wordMap)
        {
            vector<int> possible = strStr(s, iter.first);
            if (possible.empty())
            {
                return {};
            }
            if (possible.size() < candidateList.size())
            {
                candidateList = possible;
            }
        }

        vector<int> result;

        int wordSize = words[0].size();
        for (int i : candidateList)
        {
            const int left = i;
            const int right = i + wordSize;
            if (right - left == wordSize * words.size())
                result.push_back(left);
            string currentSub = s.substr(i, wordSize);

            map<string, int> wordMapCopyL = wordMap;
            wordMapCopyL[currentSub]--;
            if (left-wordSize >= 0)
                dfs(left-wordSize, right, s.substr(left-wordSize, wordSize), wordMapCopyL, result, wordSize, words.size(), s);

            map<string, int> wordMapCopyR = wordMap;
            wordMapCopyR[currentSub]--;
            if (right + wordSize <= s.length())
                dfs(left, right+wordSize, s.substr(right, wordSize), wordMapCopyR, result, wordSize, words.size(), s);
        }

        set<int> uniqueSet(result.begin(), result.end());
        return vector<int>(uniqueSet.begin(), uniqueSet.end());
    }

private:
    void dfs(int left, int right, const string& sub, map<string, int>& wordMap, vector<int>& result, const int wordSize, const int wordNum, const string& s)
    {
        if ((wordMap.find(sub) != wordMap.end()) && (wordMap[sub] != 0))
        {
            if (right - left == wordNum * wordSize)
            {
                result.push_back(left);
                return;
            }

            wordMap[sub]--;
            if (left-wordSize >= 0)
                dfs(left-wordSize, right, s.substr(left-wordSize, wordSize), wordMap, result, wordSize, wordNum, s);
            if (right + wordSize <= s.length())
                dfs(left, right+wordSize, s.substr(right, wordSize), wordMap, result, wordSize, wordNum, s);
        }
    }

    static vector<int> strStr(const string& s, const string& sub)
    {
        if (s.length() < sub.length()) {
            return {};
        }

        vector<int> answer;

        for (int i = 0; i < s.length() - sub.length() + 1; i++) {
            if (s[i] == sub[0]) {
                std::string targetString = s.substr(i, sub.length());
                if (targetString == sub) {
                    answer.push_back(i);
                }
            }
        }
        return answer;
    }
};
```
