# 374. Guess Number Higher or Lower

### Description

We are playing the Guess Game. The game is as follows:

I pick a number from 1 to n. You have to guess which number I picked.

Every time you guess wrong, I will tell you whether the number I picked is higher or lower than your guess.

You call a pre-defined API int guess(int num), which returns three possible results:

- -1: Your guess is higher than the number I picked (i.e. num > pick).
- 1: Your guess is lower than the number I picked (i.e. num < pick).
- 0: your guess is equal to the number I picked (i.e. num == pick).
Return the number that I picked.

### Example

###### Example I

```
Input: n = 10, pick = 6
Output: 6
```

###### Example II

```
Input: n = 1, pick = 1
Output: 1
```

###### Example III

```
Input: n = 2, pick = 1
Output: 1
```

### Solution

同样是二分

```c++
/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * int guess(int num);
 */

class Solution {
public:
    int guessNumber(int n) {
        int l = 0, r = n, mid, result;
        while (l <= r) {
            mid = l + (r - l) / 2;
            result = guess(mid);

            if (result == 0) return mid;
            else if (result == 1) l = mid + 1;
            else r = mid - 1;
        }
        return mid;
    }
};
```