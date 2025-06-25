# 385. Mini Parser

### Description

Given a string s represents the serialization of a nested list, implement a parser to deserialize it and return the deserialized NestedInteger.

Each element is either an integer or a list whose elements may also be integers or other lists.

### Example 

###### Example I

```
Input: s = "324"
Output: 324
Explanation: You should return a NestedInteger object which contains a single integer 324.
```

###### Example II

```
Input: s = "[123,[456,[789]]]"
Output: [123,[456,[789]]]
Explanation: Return a NestedInteger object containing a nested list with 2 elements:
1. An integer containing value 123.
2. A nested list containing two elements:
    i.  An integer containing value 456.
    ii. A nested list with one element:
         a. An integer containing value 789
```

### Solution

遍历写法

```c++
/**
 * // This is the interface that allows for creating nested lists.
 * // You should not implement it, or speculate about its implementation
 * class NestedInteger {
 *   public:
 *     // Constructor initializes an empty nested list.
 *     NestedInteger();
 *
 *     // Constructor initializes a single integer.
 *     NestedInteger(int value);
 *
 *     // Return true if this NestedInteger holds a single integer, rather than a nested list.
 *     bool isInteger() const;
 *
 *     // Return the single integer that this NestedInteger holds, if it holds a single integer
 *     // The result is undefined if this NestedInteger holds a nested list
 *     int getInteger() const;
 *
 *     // Set this NestedInteger to hold a single integer.
 *     void setInteger(int value);
 *
 *     // Set this NestedInteger to hold a nested list and adds a nested integer to it.
 *     void add(const NestedInteger &ni);
 *
 *     // Return the nested list that this NestedInteger holds, if it holds a nested list
 *     // The result is undefined if this NestedInteger holds a single integer
 *     const vector<NestedInteger> &getList() const;
 * };
 */
class Solution {
public:
    NestedInteger deserialize(string s) {
        if (s.empty()) return NestedInteger();
        
        // If it's a single integer (not a list)
        if (s[0] != '[') {
            return NestedInteger(stoi(s));
        }
        
        stack<NestedInteger> st;
        NestedInteger current;
        int i = 0;
        int n = s.size();
        
        while (i < n) {
            if (s[i] == '[') {
                st.push(NestedInteger());
                i++;
            } else if (s[i] == ']') {
                if (st.size() > 1) {
                    current = st.top();
                    st.pop();
                    st.top().add(current);
                }
                i++;
            } else if (s[i] == ',') {
                i++;
            } else {
                // Handle numbers (including negative)
                int sign = 1;
                if (s[i] == '-') {
                    sign = -1;
                    i++;
                }
                int num = 0;
                while (i < n && isdigit(s[i])) {
                    num = num * 10 + (s[i] - '0');
                    i++;
                }
                num *= sign;
                st.top().add(NestedInteger(num));
            }
        }
        
        return st.empty() ? NestedInteger() : st.top();
    }
};
```
递归写法

```c++
/**
 * // This is the interface that allows for creating nested lists.
 * // You should not implement it, or speculate about its implementation
 * class NestedInteger {
 *   public:
 *     // Constructor initializes an empty nested list.
 *     NestedInteger();
 *
 *     // Constructor initializes a single integer.
 *     NestedInteger(int value);
 *
 *     // Return true if this NestedInteger holds a single integer, rather than a nested list.
 *     bool isInteger() const;
 *
 *     // Return the single integer that this NestedInteger holds, if it holds a single integer
 *     // The result is undefined if this NestedInteger holds a nested list
 *     int getInteger() const;
 *
 *     // Set this NestedInteger to hold a single integer.
 *     void setInteger(int value);
 *
 *     // Set this NestedInteger to hold a nested list and adds a nested integer to it.
 *     void add(const NestedInteger &ni);
 *
 *     // Return the nested list that this NestedInteger holds, if it holds a nested list
 *     // The result is undefined if this NestedInteger holds a single integer
 *     const vector<NestedInteger> &getList() const;
 * };
 */
class Solution {
public:
    NestedInteger deserialize(string s) {
        int index = 0;
        return helper(s, index);
    }

private:
    NestedInteger helper(const string& s, int& index) {
        if (s[index] == '[') {
            index++;
            NestedInteger current;
            while (s[index] != ']') {
                current.add(helper(s, index));
                if (s[index] == ',') index++;
            }

            index++;
            return current;
        } else {
            int sign = s[index] == '-' ? -1 : 1;
            if (sign == -1) index++;
            int num = 0;
            while (index < s.length() && s[index] >= '0' && s[index] <= '9') {
                num = 10 * num + s[index++] - '0';
            }
            return NestedInteger(num * sign);
        }
    }
};
```
