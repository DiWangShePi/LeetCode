# 455. Assign Cookies

### Descrption

Assume you are an awesome parent and want to give your children some cookies. But, you should give each child at most one cookie.

Each child i has a greed factor g[i], which is the minimum size of a cookie that the child will be content with; and each cookie j has a size s[j]. If s[j] >= g[i], we can assign the cookie j to the child i, and the child i will be content. Your goal is to maximize the number of your content children and output the maximum number.

### Example

###### Example I

> Input: g = [1,2,3], s = [1,1]
> Output: 1
> Explanation: You have 3 children and 2 cookies. The greed factors of 3 children are 1, 2, 3. 
> And even though you have 2 cookies, since their size is both 1, you could only make the child whose greed factor is 1 content.
> You need to output 1.

###### Example II

> Input: g = [1,2], s = [1,2,3]
> Output: 2
> Explanation: You have 2 children and 3 cookies. The greed factors of 2 children are 1, 2. 
> You have 3 cookies and their sizes are big enough to gratify all of the children, 
> You need to output 2.

### Solution

我一开始的想法是对g进行排序，然后遍历s，对于每一个cookie，贪心的获取他能满足的需求最大的孩子（二分查找）。

```c++
class Solution {
public:
    int findContentChildren(vector<int>& g, vector<int>& s) {
        sort(g.begin(), g.end());
        vector<bool> contented(g.size(), false);

        int an = 0;
        for (int i = 0; i < s.size(); i++) {
            int c = s[i];
            int it = upper_bound(g.begin(), g.end(), c) - g.begin();
            if (it == g.size()) it = g.size() - 1;
            while (it > -1 && (g[it] > c || contented[it])) it--;

            if (it > -1 && it < g.size()) {
                an++;
                contented[it] = true;
            }
        }
        return an;
    }
};
```

这个解法在极端情况下是O(mn)的，可以将s也排序一下来优化。

但s也排序后，就不需要以二分查找的方式来贪心的获取能满足的孩子了。

```c++
class Solution {
public:
    int findContentChildren(vector<int>& g, vector<int>& s) {
        sort(g.begin(), g.end());
        sort(s.begin(), s.end());

        int i = 0, j = 0;
        while (i < g.size() && j < s.size()) {
            if (g[i] <= s[j]) i++;
            j++;
        }
        return i;
    }
};
```
