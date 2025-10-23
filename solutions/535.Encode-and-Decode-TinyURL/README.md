# 535. Encode and Decode TinyURL

### Description

TinyURL is a URL shortening service where you enter a URL such as https://leetcode.com/problems/design-tinyurl and it returns a short URL such as http://tinyurl.com/4e9iAk. Design a class to encode a URL and decode a tiny URL.

There is no restriction on how your encode/decode algorithm should work. You just need to ensure that a URL can be encoded to a tiny URL and the tiny URL can be decoded to the original URL.

Implement the Solution class:

- Solution() Initializes the object of the system.
- String encode(String longUrl) Returns a tiny URL for the given longUrl.
- String decode(String shortUrl) Returns the original long URL for the given shortUrl. It is guaranteed that the given shortUrl was encoded by the same object.

### Example 

###### Example I

> Input: url = "https://leetcode.com/problems/design-tinyurl"
> Output: "https://leetcode.com/problems/design-tinyurl"> 
> 
> Explanation:
> Solution obj = new Solution();
> string tiny = obj.encode(url); // returns the encoded tiny url.
> string ans = obj.decode(tiny); // returns the original url after decoding it.

### Solution

这题没什么特殊要求，只要能对应就行。

哪最简单的就是一个数字，来一个对应一个加一个。

```c++
class Solution {
public:

    // Encodes a URL to a shortened URL.
    string encode(string longUrl) {
        string re = to_string(id++);
        dict[re] = longUrl;
        return re;
    }

    // Decodes a shortened URL to its original URL.
    string decode(string shortUrl) {
        return dict[shortUrl];
    }

private:
    int id = 0;
    unordered_map<string, string> dict;
};

// Your Solution object will be instantiated and called as such:
// Solution solution;
// solution.decode(solution.encode(url));
```

> 上面的其实不是很标准，虽然检查少了会比较快

```c++
class Solution {
private:
    unordered_map<string, string> urlMap;
    int id = 0;
    const string PREFIX = "http://tinyurl.com/";

public:
    // Encodes a URL to a shortened URL.
    string encode(string longUrl) {
        string key = to_string(id++);
        urlMap[key] = longUrl;
        return PREFIX + key;
    }

    // Decodes a shortened URL to its original URL.
    string decode(string shortUrl) {
        string key = shortUrl.substr(PREFIX.length());
        return urlMap[key];
    }
};

// Your Solution object will be instantiated and called as such:
// Solution solution;
// solution.decode(solution.encode(url));
```

更合适的做法是用哈希

```c++
class Solution {
private:
    unordered_map<string, string> urlMap;
    const string PREFIX = "http://tinyurl.com/";
    const string CHAR_POOL = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    const int KEY_LEN = 6;

    mt19937 rng;
    uniform_int_distribution<int> dist;
    
public:
    Solution() : rng(time(nullptr)), dist(0, CHAR_POOL.length() - 1) {}
    
    string encode(string longUrl) {
        string key;
        do {
            key = generateRandomKey();
        } while (urlMap.find(key) != urlMap.end());
        
        urlMap[key] = longUrl;
        return PREFIX + key;
    }
    
    string decode(string shortUrl) {
        string key = shortUrl.substr(PREFIX.length());
        return urlMap[key];
    }
    
private:
    string generateRandomKey() {
        string key;
        for (int i = 0; i < KEY_LEN; i++) {
            key += CHAR_POOL[dist(rng)];
        }
        return key;
    }
};

// Your Solution object will be instantiated and called as such:
// Solution solution;
// solution.decode(solution.encode(url));
```
