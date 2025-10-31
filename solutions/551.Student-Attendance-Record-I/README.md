# 551. Student Attendance Record I

**Tags:** String

### Description

You are given a string s representing an attendance record for a student where each character signifies whether the student was absent, late, or present on that day. The record only contains the following three characters:

- 'A': Absent.
- 'L': Late.
- 'P': Present.
The student is eligible for an attendance award if they meet both of the following criteria:

- The student was absent ('A') for strictly fewer than 2 days total.
- The student was never late ('L') for 3 or more consecutive days.
Return true if the student is eligible for an attendance award, or false otherwise.

### Example

###### Example I

> Input: s = "PPALLP"
> Output: true
> Explanation: The student has fewer than 2 absences and was never late 3 or more consecutive days.

###### Example II

> Input: s = "PPALLL"
> Output: false
> Explanation: The student was late 3 consecutive days in the last 3 days, so is not eligible for the award.

### Solution

遍历，检查这两个条件是否满足

```c++
class Solution {
public:
    bool checkRecord(string s) {
        int a = 0;
        for (char c : s) {
            if (c == 'A') a++;
        }
        if (a > 1) return false;

        int l = 0, an = INT_MIN;
        for (char c : s) {
            if (c == 'L') l++;
            else l = 0;
            an = max(l, an);
        }
        if (an >= 3) return false;
        return true;
    }
};
```
