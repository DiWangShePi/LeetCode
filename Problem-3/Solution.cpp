using namespace std;

#include <iostream>
#include <map>

class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        map<char, int> hashMap;
        int i = 0, j = 0, lengthOfLongest =0;

        char currentChar;
        for ( ; i < s.length(); i++)
        {
            currentChar = s[i];
            // cout << "Current Char is : " << currentChar << endl;
            
            if (hashMap.count(currentChar) > 0)
            {
                lengthOfLongest = (lengthOfLongest < (i - j)) ? (i - j) : lengthOfLongest;
                j = (j <= hashMap[currentChar]) ? (hashMap[currentChar] + 1) : j ;
            }
            hashMap[currentChar] = i;
        }

        // cout << "Size of the final hashMap is : " << hashMap.size() << endl;
        // cout << "Value of the final J is: " << j << endl;

        lengthOfLongest = (lengthOfLongest < (s.length() - j)) ? (s.length() - j) : lengthOfLongest;
        return lengthOfLongest;
    }
};


int main() {
    Solution solution;
    int result = solution.lengthOfLongestSubstring("abcabcbb");
    cout << "The result is : " << result << endl;
}