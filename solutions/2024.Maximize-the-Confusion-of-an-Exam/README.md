# 2024. Maximize the Confusion of an Exam

**Tags:** Sliding Window

### Description

A teacher is writing a test with n true/false questions, with 'T' denoting true and 'F' denoting false. He wants to confuse the students by maximizing the number of consecutive questions with the same answer (multiple trues or multiple falses in a row).

You are given a string answerKey, where answerKey[i] is the original answer to the ith question. In addition, you are given an integer k, the maximum number of times you may perform the following operation:

- Change the answer key for any question to 'T' or 'F' (i.e., set answerKey[i] to 'T' or 'F').
Return the maximum number of consecutive 'T's or 'F's in the answer key after performing the operation at most k times.

### Example

###### Example I

> Input: answerKey = "TTFF", k = 2
> Output: 4
> Explanation: We can replace both the 'F's with 'T's to make answerKey = "TTTT".
> There are four consecutive 'T's.

###### Example II

> Input: answerKey = "TFFT", k = 1
> Output: 3
> Explanation: We can replace the first 'T' with an 'F' to make answerKey = "FFFT".
> Alternatively, we can replace the second 'T' with an 'F' to make answerKey = "TFFF".
> In both cases, there are three consecutive 'F's.

###### Example III

> Input: answerKey = "TTFTTFTT", k = 1
> Output: 5
> Explanation: We can replace the first 'F' to make answerKey = "TTTTTFTT"
> Alternatively, we can replace the second 'F' to make answerKey = "TTFTTTTT". 
> In both cases, there are five consecutive 'T's.

### Solution

分别考虑修改 T 和修改 F 能得到的最长连续字符串，取最大值。

```c++
class Solution {
public:
    int maxConsecutiveAnswers(string answerKey, int k) {
        return max(count(answerKey, k, 'T'), count(answerKey, k, 'F'));
    }

private:
    int count(string& answerKey, int k, char c) {
        int an = 0, l = 0, count = 0;
        for (int i = 0; i < answerKey.size(); i++) {
            if (answerKey[i] != c) {
                count++;
                while (count > k) {
                    if (answerKey[l++] != c) count--;
                }
            }
            
            an = max(an, i - l + 1);
        }
        return an;
    }
};
```
