class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        hashMap = {}
        j = lengthOfLongest = 0
        
        for i, char in enumerate(s):
            if char in hashMap:
                lengthOfLongest = max(lengthOfLongest, i - j)
                j = max(j, hashMap[char] + 1)
            hashMap[char] = i

        return max(lengthOfLongest, len(s) - j)
