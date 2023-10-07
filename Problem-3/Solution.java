import java.util.HashMap

class Solution {
    public int lengthOfLongestSubstring(String s) {
        HashMap<Character, Integer> hashMap = new HashMap<>();
        int i = 0, j = 0, lengthOfLongest = 0;

        char currentChar;
        for ( ; i < s.length(); i++) {
            currentChar = s.charAt(i);

            if (hashMap.containsKey(currentChar)) {
                lengthOfLongest = (lengthOfLongest < (i - j)) ? (i - j) : lengthOfLongest;
                j = (j <= hashMap.get(currentChar)) ? (hashMap.get(currentChar) + 1) : j ;
            }
            hashMap.put(currentChar, i);
        }

        return (lengthOfLongest < (s.length() - j)) ? (s.length() - j) : lengthOfLongest;
    }
}