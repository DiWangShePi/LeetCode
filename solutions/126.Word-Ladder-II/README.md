# 126. Word Ladder II

### Description

A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:

Every adjacent pair of words differs by a single letter.
Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
sk == endWord
Given two words, beginWord and endWord, and a dictionary wordList, return all the shortest transformation sequences from beginWord to endWord, or an empty list if no such sequence exists. Each sequence should be returned as a list of the words [beginWord, s1, s2, ..., sk].

### Solution

和下一题一样，构建树，然后广度优先搜索。

### Implementation

###### c++

```c++
class Solution {
public:
    vector<vector<string> >ans;
    unordered_map<string,int>mp;
     string b;
    void dfs(string word, vector<string>&str){
        string current = str.back();
        if(word==b){
                reverse(str.begin(),str.end());
                ans.push_back(str);
                reverse(str.begin(),str.end());
            return;
        }
        int currentLevel=mp[current];
        int m=word.length();
        for(int i=0;i<m;i++){
            char original=current[i];
            for(char j='a';j<='z';j++){
                if(j!=original){
                    current[i]=j;
                    if(mp.find(current)!=mp.end() && mp[current] == currentLevel-1){
                        str.push_back(current);
                        dfs(current,str);
                        str.pop_back();
                    }
                }
            }
            current[i]=original;
        }
    }
    vector<vector<string>> findLadders(string beginWord, string endWord, vector<string>& wordList) {
       unordered_set<string> st(wordList.begin(), wordList.end());

        // Perform BFS traversal and push the string in the queue
        // as soon as they’re found in the wordList.
        queue<string> q;
        b = beginWord;
        q.push({beginWord});

        // beginWord initialised with level 1.
        mp[beginWord] = 1;
        int m = beginWord.size();
        st.erase(beginWord);
        bool flag = false;
        while(!q.empty() && !flag){
                string current = q.front();
            int level=mp[current];
                q.pop();
                for(int j=0;j<m && !flag;j++){
                    char originalChar = current[j];
                    for(int i='a';i<='z' && !flag;i++){
                        if(i!=originalChar){
                            current[j]=i;
                            if(st.find(current)!=st.end()){
                                mp[current]=level+1;
                                if(current==endWord){
                                    flag=true;
                                    break;
                                }
                                
                                st.erase(current);
                                q.push(current);
                            }
                    }
                    }
                    current[j]=originalChar;
                }
        }
        vector<string>str;
        str.push_back(endWord);
        dfs(endWord,str);
        return ans;
        
    }
};
```
