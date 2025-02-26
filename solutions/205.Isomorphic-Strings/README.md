# 205. Isomorphic Strings

### Description

Given two strings s and t, determine if they are isomorphic.

Two strings s and t are isomorphic if the characters in s can be replaced to get t.

All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.

### Solution

创建字典并遍历字符串，用s中的字符作为键，用t中的字符作为值。

- 若当前字符在字典中不存在，则据此创建新的键值对
- 若存在，检测是否一样。不一样则返回false，一样则继续。

### Implementation

###### c++

```c++
class Solution {
public:
    bool isIsomorphic(string s, string t) {
        if (s.length() != t.length()) {
            return false;
        }

        std::unordered_map<char, char> map1; // s -> t
        std::unordered_map<char, char> map2; // t -> s

        for (int i = 0; i < s.length(); ++i) {
            char char_s = s[i];
            char char_t = t[i];

            if (map1.count(char_s) > 0) {
                if (map1[char_s] != char_t) return false; 
            } else map1[char_s] = char_t;

            if (map2.count(char_t) > 0) {
                if (map2[char_t] != char_s) return false; 
            } else map2[char_t] = char_s;
        }
        return true;
    }
};
```

> Another solution I find interesting

```c++
class Solution {
public:
    bool isIsomorphic(string s, string t) {
        if(s==t) return true;
        int hash[256]={0}; //hash array to map s[i] with t[i]
        bool istCharMapped[256]={0}; //checks if t[i] is already mapped with s[i]
        for(int i=0;i<s.size();i++){
            if(hash[s[i]]==0 && istCharMapped[t[i]]==0 ){ //check for bidirectional mapping
                //char not mapped -> start mapping
                hash[s[i]]=t[i];
                istCharMapped[t[i]]=true;
            }
        }
        //if successful mapping happened we can make string t from string s using hash array
        for(int i=0;i<s.size();i++){
            if(hash[s[i]]!=t[i]){
                return false;
            }
        }
        return true;
    }
};
```