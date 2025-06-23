# 352. Data Stream as Disjoint Intervals

### Description

Given a data stream input of non-negative integers a1, a2, ..., an, summarize the numbers seen so far as a list of disjoint intervals.

Implement the SummaryRanges class:

- SummaryRanges() Initializes the object with an empty stream.
- void addNum(int value) Adds the integer value to the stream.
- int[][] getIntervals() Returns a summary of the integers in the stream currently as a list of disjoint intervals [starti, endi]. The answer should be sorted by starti.

### Example 

###### Example I

```
Input
["SummaryRanges", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals"]
[[], [1], [], [3], [], [7], [], [2], [], [6], []]
Output
[null, null, [[1, 1]], null, [[1, 1], [3, 3]], null, [[1, 1], [3, 3], [7, 7]], null, [[1, 3], [7, 7]], null, [[1, 3], [6, 7]]]

Explanation
SummaryRanges summaryRanges = new SummaryRanges();
summaryRanges.addNum(1);      // arr = [1]
summaryRanges.getIntervals(); // return [[1, 1]]
summaryRanges.addNum(3);      // arr = [1, 3]
summaryRanges.getIntervals(); // return [[1, 1], [3, 3]]
summaryRanges.addNum(7);      // arr = [1, 3, 7]
summaryRanges.getIntervals(); // return [[1, 1], [3, 3], [7, 7]]
summaryRanges.addNum(2);      // arr = [1, 2, 3, 7]
summaryRanges.getIntervals(); // return [[1, 3], [7, 7]]
summaryRanges.addNum(6);      // arr = [1, 2, 3, 6, 7]
summaryRanges.getIntervals(); // return [[1, 3], [6, 7]]
```

### Solution

我一开始的想法是保留一个字典，将新加入的数字记录下来，然后每次查询区间的时候逐个检查。
实现上将检查的长度写死了，所以这可以说是常数的时间复杂度，但还是非常慢。

```c++
class SummaryRanges {
    unordered_map<int, int> dict;
public:
    SummaryRanges() {}
    
    void addNum(int value) {
        dict[value] = 1;
    }
    
    vector<vector<int>> getIntervals() {
        vector<vector<int>> an;
        vector<int> current;
        for (int i = 0; i<= 10000; i++) {
            if (dict.count(i) != 0) {
                if (current.empty()) {
                    current.push_back(i); current.push_back(i);
                } else {
                    current.pop_back(); current.push_back(i);
                }
                
            }

            if (dict.count(i + 1) == 0 && !current.empty()) {
                an.push_back(current);
                current.clear();
            }
        }

        return an;
    }
};

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * SummaryRanges* obj = new SummaryRanges();
 * obj->addNum(value);
 * vector<vector<int>> param_2 = obj->getIntervals();
 */
```

我们不妨考虑一下更方便的做法，比如将数字记录在一个列表里（反正没有删除）。虽然这样理论上的复杂度会低一些，但实际运行的开销似乎没降低多少。

```c++
class SummaryRanges {
    vector<int> values;
    unordered_map<int, int> dict; 
public:
    SummaryRanges() {}
    
    void addNum(int value) {
        if (dict[value] == 1) return;

        values.push_back(value);
        dict[value] = 1;
    }
    
    vector<vector<int>> getIntervals() {
        sort(values.begin(), values.end());

        vector<vector<int>> an;
        vector<int> current;
        for (int value : values) {
            if (current.empty()) {
                current.push_back(value); current.push_back(value);
            } else {
                int n = current.size();
                if (current[n - 1] + 1 == value) {
                    current.pop_back();
                    current.push_back(value);
                } else {
                    an.push_back(current);
                    current.clear();

                    current.push_back(value); current.push_back(value);
                }
            }
        }

        if (!current.empty()) an.push_back(current);
        return an;
    }
};

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * SummaryRanges* obj = new SummaryRanges();
 * obj->addNum(value);
 * vector<vector<int>> param_2 = obj->getIntervals();
 */
```

官解的做法是维护上下区间，每次加入数字的时候查找是否存在可以扩展的区间。

```c++
class SummaryRanges {
private:
    map<int, int> intervals;

public:
    SummaryRanges() {}
    
    void addNum(int val) {
        // 找到 l1 最小的且满足 l1 > val 的区间 interval1 = [l1, r1]
        // 如果不存在这样的区间，interval1 为尾迭代器
        auto interval1 = intervals.upper_bound(val);
        // 找到 l0 最大的且满足 l0 <= val 的区间 interval0 = [l0, r0]
        // 在有序集合中，interval0 就是 interval1 的前一个区间
        // 如果不存在这样的区间，interval0 为尾迭代器
        auto interval0 = (interval1 == intervals.begin() ? intervals.end() : prev(interval1));

        if (interval0 != intervals.end() && interval0->first <= val && val <= interval0->second) {
            // 情况一
            return;
        }
        else {
            bool left_aside = (interval0 != intervals.end() && interval0->second + 1 == val);
            bool right_aside = (interval1 != intervals.end() && interval1->first - 1 == val);
            if (left_aside && right_aside) {
                // 情况四
                int left = interval0->first, right = interval1->second;
                intervals.erase(interval0);
                intervals.erase(interval1);
                intervals.emplace(left, right);
            }
            else if (left_aside) {
                // 情况二
                ++interval0->second;
            }
            else if (right_aside) {
                // 情况三
                int right = interval1->second;
                intervals.erase(interval1);
                intervals.emplace(val, right);
            }
            else {
                // 情况五
                intervals.emplace(val, val);
            }
        }
    }
    
    vector<vector<int>> getIntervals() {
        vector<vector<int>> ans;
        for (const auto& [left, right]: intervals) {
            ans.push_back({left, right});
        }
        return ans;
    }
};
```