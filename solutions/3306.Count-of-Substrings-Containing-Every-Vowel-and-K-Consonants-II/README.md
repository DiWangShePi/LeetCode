# 3306. Count of Substrings Containing Every Vowel and K Consonants II

**Tags:** Sliding Window, DO IT AGAIN

### Description

You are given a string word and a non-negative integer k.

Return the total number of substrings of word that contain every vowel ('a', 'e', 'i', 'o', and 'u') at least once and exactly k consonants.

### Example

###### Example I

> Input: word = "aeioqq", k = 1
> Output: 0
> Explanation:
> There is no substring with every vowel.

###### Example II

> Input: word = "aeiou", k = 0
> Output: 1
> Explanation:
> The only substring with every vowel and zero consonants is word[0..4], which is "aeiou".

###### Example III

> Input: word = "ieaouqqieaouqq", k = 1
> Output: 3
> Explanation:
> The substrings with every vowel and one consonant are:
> - word[0..5], which is "ieaouq".
> - word[6..11], which is "qieaou".
> - word[7..12], which is "ieaouq".

### Solution

求解5个元音，且辅音出现最少 K 个的字串个数，减去5个元音，且辅音出现最少 K + 1 个的字串个数。

```c++
class Solution {
    const string VOWEL = "aeiou";
    
public:
    long long countOfSubstrings(string word, int k) {
        return count(word, k) - count(word, k + 1);
    }

private:
    long long count(string& word, int k) {
        long long ans = 0;
        unordered_map<char, int> cnt1;
        int cnt2 = 0, left = 0; 
        for (char b : word) {
            if (VOWEL.find(b) != string::npos)cnt1[b]++;
            else cnt2++;

            while (cnt1.size() == 5 && cnt2 >= k) {
                char out = word[left];
                if (VOWEL.find(out) != string::npos) {
                    if (--cnt1[out] == 0) {
                        cnt1.erase(out);
                    }
                } else {
                    cnt2--;
                }
                left++;
            }
            ans += left;
        }
        return ans;
    }
};
```
