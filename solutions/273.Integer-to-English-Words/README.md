# 273. Integer to English Words

### Description

Convert a non-negative integer num to its English words representation.

### Example

###### Example 1:

```
Input: num = 123
Output: "One Hundred Twenty Three"
```

###### Example 2:

```
Input: num = 12345
Output: "Twelve Thousand Three Hundred Forty Five"
```

###### Example 3:

```
Input: num = 1234567
Output: "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
```

### Solution

2^31 = 2,147,483,648

将数字每三位分成一部分，分别处理，处理完加上十亿、百万、千的表示即可。

三位数考虑三部分，小于20的按对照表，大于二十小于一百的模十后分表查表，大于一百的模一百再按前面的方式处理。

```c++
class Solution {

public:
    string numberToWords(int num) {
        if (num == 0) return "Zero";

        const std::vector<std::pair<int, std::string>> units = {
            {1000000000, "Billion"},
            {1000000,    "Million"},
            {1000,       "Thousand"},
            {1,          ""}
        };

        std::string result;
        for (const auto& [divisor, label] : units) {
            if (num >= divisor) {
                int chunk = num / divisor;
                num %= divisor;
                result += helper(chunk) + (label.empty() ? "" : " " + label) + " ";
            }
        }

        while (!result.empty() && result.back() == ' ')
            result.pop_back();

        return result;
    }

private:
    std::string helper(int num) {
        static const char* below_20[] = {
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
            "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen",
            "Sixteen", "Seventeen", "Eighteen", "Nineteen"
        };

        static const char* tens[] = {
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"
        };

        std::string result;

        if (num >= 100) {
            result += std::string(below_20[num / 100]) + " Hundred";
            num %= 100;
            if (num > 0) result += " ";
        }

        if (num >= 20) {
            result += std::string(tens[num / 10]);
            num %= 10;
            if (num > 0) result += " ";
        }

        if (num > 0 && num < 20) {
            result += below_20[num];
        }

        return result;
    }
};
```
