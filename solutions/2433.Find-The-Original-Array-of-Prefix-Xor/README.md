# 2433. Find The Original Array of Prefix Xor

**Tags:** Bitwise operation

### Description

You are given an integer array pref of size n. Find and return the array arr of size n that satisfies:

- pref[i] = arr[0] ^ arr[1] ^ ... ^ arr[i].
Note that ^ denotes the bitwise-xor operation.

It can be proven that the answer is unique.

### Example

###### Example I

> Input: pref = [5,2,0,3,1]
> Output: [5,7,2,3,2]
> Explanation: From the array [5,7,2,3,2] we have the following:
> - pref[0] = 5.
> - pref[1] = 5 ^ 7 = 2.
> - pref[2] = 5 ^ 7 ^ 2 = 0.
> - pref[3] = 5 ^ 7 ^ 2 ^ 3 = 3.
> - pref[4] = 5 ^ 7 ^ 2 ^ 3 ^ 2 = 1.

###### Example II

> Input: pref = [13]
> Output: [13]
> Explanation: We have pref[0] = arr[0] = 13.

### Solution

pref[i] ^ pref[i-1] = an[i]

```c++
class Solution {
public:
    vector<int> findArray(vector<int>& pref) {
        vector<int> arr = pref;
        for (int i = pref.size() - 1; i > 0; i--) {
            arr[i] ^= pref[i-1];
        }
        return arr;
    }
};
```
