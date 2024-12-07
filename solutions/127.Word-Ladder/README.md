# 127. Word Ladder

### Description

A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:

Every adjacent pair of words differs by a single letter.
Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
sk == endWord
Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.

### Solution

将节点抽象为图，随后广度优先搜索。建立连接的过程可以采用虚拟节点，而非遍历枚举。

> 如对于 hit 这一单词，构建 *it, h*t, hi* 三个虚拟节点，并构建连接他们的边。若另一个单词可以经过一次转换成为hit，则它必可以与这三个单词中的一个构成连接。

对于每一个节点，额外定义一个bool类型的属性，用于指示该值是否被遍历到了。这是为了实现双向广度优先搜索。

到这一步再回顾解题思路，其实节点并不是必要的，纯粹用字符串去做搜索也是可以的。

### Implementation

###### c++

```c++
class Solution {
public:
    int ladderLength(string beginWord, string endWord, vector<string>& wordList) {
        unordered_set<string> dict(wordList.begin(), wordList.end()); 
        if (!dict.count(endWord)) return 0; 

        unordered_set<string> begins, ends; 
        begins.insert(beginWord); 
        ends.insert(endWord); 

        int ladder = 1;  
        while (!begins.empty() && !ends.empty()) {
            if (begins.size() > ends.size()) {
                swap(begins, ends); 
            }

            unordered_set<string> temp; 
            for (string s : begins) {
                for (int i = 0 ; i < s.size() ; i++) {
                    char orig = s[i]; 
                    for (char ch = 'a' ; ch <= 'z' ; ch++) {
                        if (ch == orig) continue; 
                        s[i] = ch; 
                        if (ends.count(s)) return ladder + 1; 
                        if (dict.count(s)) {
                            dict.erase(s); 
                            temp.insert(s); 
                        }
                        s[i] = orig; 
                    }
                }
            }

            begins = temp; 
            ladder++; 
        }

        return 0; 
    }
};
```