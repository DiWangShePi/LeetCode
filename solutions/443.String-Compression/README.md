# 443. String Compression

### Description

Given an array of characters chars, compress it using the following algorithm:

Begin with an empty string s. For each group of consecutive repeating characters in chars:

- If the group's length is 1, append the character to s.
- Otherwise, append the character followed by the group's length.
The compressed string s should not be returned separately, but instead, be stored in the input character array chars. Note that group lengths that are 10 or longer will be split into multiple characters in chars.

After you are done modifying the input array, return the new length of the array.

You must write an algorithm that uses only constant extra space.

### Example 

###### Example I

> Input: chars = ["a","a","b","b","c","c","c"]
> Output: Return 6, and the first 6 characters of the input array should be: ["a","2","b","2","c","3"]
> Explanation: The groups are "aa", "bb", and "ccc". This compresses to "a2b2c3".

###### Example II

> Input: chars = ["a"]
> Output: Return 1, and the first character of the input array should be: ["a"]
> Explanation: The only group is "a", which remains uncompressed since it's a single character.

###### Example III

> Input: chars = ["a","b","b","b","b","b","b","b","b","b","b","b","b"]
> Output: Return 4, and the first 4 characters of the input array should be: ["a","b","1","2"].
> Explanation: The groups are "a" and "bbbbbbbbbbbb". This compresses to "ab12".

### Solution

按要求实现即可

```c++
class Solution {
public:
    int compress(vector<char>& chars) {
        int n = chars.size();
        if (n == 0) return 0;

        char last_c = chars[0];
        int count = 1, size = 0;
        for (int i = 1; i < n; i++) {
            if (chars[i] == last_c) count++;
            else {
                chars[size++] = last_c;
                if (count != 1) {
                    string c_to_s = to_string(count);
                    for (char c : c_to_s) chars[size++] = c;
                    count = 1;
                }
                last_c = chars[i];
            }
        }

        // update last char
        chars[size++] = last_c;
        if (count != 1) {
            string c_to_s = to_string(count);
            for (char c : c_to_s) chars[size++] = c;
            count = 1;
        }
        return size;
    }
};
```