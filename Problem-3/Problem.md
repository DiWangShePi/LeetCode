### Problem Description
Given String s, find the length of the longest substring without repeating characters.

#### Example I
Input: s = "abcabcbb"
Output: 3 

#### Example II
Input: s = "bbbbb"
Output: 1

#### Example III
Input: s = "pwwkew"
Output: 3

### Solution I
Traverse the string and initialize a hash table. For each character encountered, retrieve whether the character exists in the hash table. If it does not exist, record the subscript of the current character and traverse backward. If it exists, take the larger value of the longest word string size and the current hash table size, update the subscript i to the next of the previous repeated character, clear the hash table and continue.

### Solution II
Based on the previous solution, traversal is not necessary because we can confirm that there are no duplicate characters in the current substring.