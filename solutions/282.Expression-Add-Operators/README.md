# 282. Expression Add Operators

### Description

Given a string num that contains only digits and an integer target, return all possibilities to insert the binary operators '+', '-', and/or '*' between the digits of num so that the resultant expression evaluates to the target value.

Note that operands in the returned expressions should not contain leading zeros.

### Example 

###### Example I

```
Input: num = "123", target = 6
Output: ["1*2*3","1+2+3"]
Explanation: Both "1*2*3" and "1+2+3" evaluate to 6.
```

###### Example II

```
Input: num = "232", target = 8
Output: ["2*3+2","2+3*2"]
Explanation: Both "2*3+2" and "2+3*2" evaluate to 8.
```

###### Example III

```
Input: num = "3456237490", target = 9191
Output: []
Explanation: There are no expressions that can be created from "3456237490" to evaluate to 9191.
```

### Solution

万般皆下品，唯有暴力高

```c++
class Solution {
public:
    vector<string> addOperators(string num, int target) {
        vector<string> res;
        dfs(num, target, 0, "", 0, 0, res);
        return res;
    }

private:
    void dfs(const string& num, int target, int pos,
             string expr, long long curr_val, long long prev_num,
             vector<string>& res) {

        if (pos == num.size()) {
            if (curr_val == target) {
                res.push_back(expr);
            }
            return;
        }

        for (int i = pos; i < num.size(); ++i) {
            if (i != pos && num[pos] == '0') break;

            string part = num.substr(pos, i - pos + 1);
            long long n = stoll(part);  

            if (pos == 0) {
                dfs(num, target, i + 1, part, n, n, res);
            } else {
                // +
                dfs(num, target, i + 1, expr + "+" + part, curr_val + n, n, res);
                // -
                dfs(num, target, i + 1, expr + "-" + part, curr_val - n, -n, res);
                // *
                dfs(num, target, i + 1, expr + "*" + part, curr_val - prev_num + prev_num * n, prev_num * n, res);
            }
        }
    }
};
```