# 3889. Mirror Frequency Distance

**Tags:** 

### Description

You are given a string s consisting of lowercase English letters and digits.

For each character, its mirror character is defined by reversing the order of its character set:

For letters, the mirror of a character is the letter at the same position from the end of the alphabet.
For example, the mirror of 'a' is 'z', and the mirror of 'b' is 'y', and so on.
For digits, the mirror of a character is the digit at the same position from the end of the range '0' to '9'.
For example, the mirror of '0' is '9', and the mirror of '1' is '8', and so on.
For each unique character c in the string:

Let m be its mirror character.
Let freq(x) denote the number of times character x appears in the string.
Compute the absolute difference between their frequencies, defined as: |freq(c) - freq(m)|
The mirror pairs (c, m) and (m, c) are the same and must be counted only once.

Return an integer denoting the total sum of these values over all such distinct mirror pairs.

### Example

###### Example I

> Input: s = "ab1z9"
> Output: 3
> Explanation:
> For every mirror pair:
> c	m	freq(c)	freq(m)	|freq(c) - freq(m)|
> a	z	1	1	0
> b	y	1	0	1
> 1	8	1	0	1
> 9	0	1	0	1
> Thus, the answer is 0 + 1 + 1 + 1 = 3.

###### Example II

> Input: s = "4m7n"
> Output: 2
> Explanation:
> c	m	freq(c)	freq(m)	|freq(c) - freq(m)|
> 4	5	1	0	1
> m	n	1	1	0
> 7	2	1	0	1
> Thus, the answer is 1 + 0 + 1 = 2.​​​​​​​

###### Example III

> Input: s = "byby"
> Output: 0
> Explanation:
> c	m	freq(c)	freq(m)	|freq(c) - freq(m)|
> b	y	2	2	0
> Thus, the answer is 0.

### Solution

按要求实现即可

```c++
class Solution {
public:
    int mirrorFrequency(string s) {
        unordered_map<int, int> dictC;
        unordered_map<int, int> dictN;
        for (char c : s) {
            if (c - 'a' < 0) dictN[c - '0']++;
            else dictC[c - 'a']++;
        }

        int ans = 0;
        for (int i = 0; i < 26; i++) {
            // cout << dictC[i] << " " << dictC[25 - i] << " " << abs(dictC[i] - dictC[25 - i]) << endl;
            ans += abs(dictC[i] - dictC[25 - i]);
            dictC[i] = 0;
            dictC[25 - i] = 0;
        }
        for (int i = 0; i < 10; i++) {
            // cout << dictN[i] << " " << dictN[9 - i] << " " << abs(dictN[i] - dictN[9 - i]) << endl;
            ans += abs(dictN[i] - dictN[9 - i]);
            dictN[i] = 0;
            dictN[9 - i] = 0;
        }
        return ans;
    }
};
```
