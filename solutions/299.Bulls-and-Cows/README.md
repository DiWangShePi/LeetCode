# 299. Bulls and Cows

### Description

You are playing the Bulls and Cows game with your friend.

You write down a secret number and ask your friend to guess what the number is. When your friend makes a guess, you provide a hint with the following info:

- The number of "bulls", which are digits in the guess that are in the correct position.
- The number of "cows", which are digits in the guess that are in your secret number but are located in the wrong position. Specifically, the non-bull digits in the guess that could be rearranged such that they become bulls.
Given the secret number secret and your friend's guess guess, return the hint for your friend's guess.

The hint should be formatted as "xAyB", where x is the number of bulls and y is the number of cows. Note that both secret and guess may contain duplicate digits.

### Example 

###### Example I

```
Input: secret = "1807", guess = "7810"
Output: "1A3B"
Explanation: Bulls are connected with a '|' and cows are underlined:
"1807"
  |
"7810"
```

###### Example II

```
Input: secret = "1123", guess = "0111"
Output: "1A1B"
Explanation: Bulls are connected with a '|' and cows are underlined:
"1123"        "1123"
  |      or     |
"0111"        "0111"
Note that only one of the two unmatched 1s is counted as a cow since the non-bull digits can only be rearranged to allow one 1 to be a bull.
```

### Solution

由于bull的优先级比cow高，我们先遍历一遍，逐个比较得出bull的个数。

没对上时，将secret中的值放入一个字典。将guess中没对上的值与字典的键值比较，以此获取cow的值。

```c++
class Solution {
public:
    string getHint(string secret, string guess) {
        int x = 0;
        unordered_map<char, int> dict;
        vector<char> guess_array;
        for (int i = 0; i < secret.size(); i++) {
            if (secret[i] == guess[i]) x++;
            else {
                dict[secret[i]]++;
                guess_array.push_back(guess[i]);
            }
        }
        int y = 0;
        for (int i = 0; i < guess_array.size(); i++) {
            char current = guess_array[i];
            if (dict.count(current) != 0 && dict[current] != 0) {
                y++;
                dict[current]--;
            }
        }
        return to_string(x) + "A" + to_string(y) + "B";
    }
};
```
