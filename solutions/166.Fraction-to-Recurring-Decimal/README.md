# 166. Fraction to Recurring Decimal

### Descirpition

Given two integers representing the numerator and denominator of a fraction, return the fraction in string format.

If the fractional part is repeating, enclose the repeating part in parentheses.

If multiple answers are possible, return any of them.

It is guaranteed that the length of the answer string is less than 104 for all the given inputs.

### Solution

做除法，参考：https://leetcode.cn/problems/fraction-to-recurring-decimal/solutions/1028368/fen-shu-dao-xiao-shu-by-leetcode-solutio-tqdw/

### Implementation

###### c++

```c++
class Solution {
public:
    string fractionToDecimal(int numerator, int denominator) {
        long long numeratorLong = numerator;
        long long denominatorLong = denominator;

        // If the fraction is divisible without remainder, return the integer part
        if (numeratorLong % denominatorLong == 0) {
            return to_string(numeratorLong / denominatorLong);
        }

        // Initialize result string
        string ans;

        // Handle negative signs
        if (numeratorLong < 0 ^ denominatorLong < 0) {
            ans.push_back('-');
        }

        // Use absolute values for integer and fractional parts
        numeratorLong = abs(numeratorLong);
        denominatorLong = abs(denominatorLong);

        // Integer part
        long long integerPart = numeratorLong / denominatorLong;
        ans += to_string(integerPart);
        ans.push_back('.');

        // Fractional part
        unordered_map<long long, int> remainderMap;
        long long remainder = numeratorLong % denominatorLong;
        string fractionPart;
        
        while (remainder != 0 && !remainderMap.count(remainder)) {
            remainderMap[remainder] = fractionPart.length();
            remainder *= 10;
            fractionPart += to_string(remainder / denominatorLong);
            remainder %= denominatorLong;
        }

        // If a remainder repeats, insert parentheses
        if (remainder != 0) {
            int repeatIndex = remainderMap[remainder];
            fractionPart.insert(repeatIndex, 1, '(');
            fractionPart.push_back(')');
        }

        ans += fractionPart;
        return ans;
    }
};
```
