#include <map>
#include <iostream>

using namespace std;

class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        std::map<char, int> dict;
        int maxLength = 0, left = 0;

        for (int i = 0; i < s.length(); i++) {
            char currentChar = s[i];

            if (dict.count(currentChar) > 0) {
                left = dict[currentChar] + 1 > left ? dict[currentChar] + 1 : left;
            }

            dict[currentChar] = i;
            maxLength = (i - left + 1) > maxLength ? (i - left + 1) : maxLength;
        }
        return maxLength;
    }
};