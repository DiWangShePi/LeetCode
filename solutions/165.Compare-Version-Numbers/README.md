# 165. Compare Version Numbers

### Description

Given two version strings, version1 and version2, compare them. A version string consists of revisions separated by dots '.'. The value of the revision is its integer conversion ignoring leading zeros.

To compare version strings, compare their revision values in left-to-right order. If one of the version strings has fewer revisions, treat the missing revision values as 0.

Return the following:

- If version1 < version2, return -1.
- If version1 > version2, return 1.
- Otherwise, return 0.

### Solution

按照"."分割字符串，从左往右逐个将字符串转变为数字进行比对，长度不够用0替代。

### Implementation

###### c++

```c++
class Solution {
public:
    int compareVersion(string version1, string version2) {
        vector<int> v1 = splitVersion(version1);
        vector<int> v2 = splitVersion(version2);

        int maxLength = max(v1.size(), v2.size());
        for (int i = 0; i < maxLength; ++i) {
            int rev1 = i < v1.size() ? v1[i] : 0;
            int rev2 = i < v2.size() ? v2[i] : 0;
            if (rev1 < rev2) return -1;
            if (rev1 > rev2) return 1;
        }
        return 0; 
    }

private:
    vector<int> splitVersion(const string& version) {
        vector<int> result;
        stringstream ss(version);
        string part;
        while (getline(ss, part, '.')) {
            result.push_back(stoi(part));
        }
        return result;
    }
};
```