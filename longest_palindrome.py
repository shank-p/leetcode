"""
    409. Longest Palindrome
    -> leetcode (easy)
    https://leetcode.com/problems/longest-palindrome/description/?envType=daily-question&envId=2024-06-04
"""


from collections import Counter

class Solution(object):
    """Return length of the longest possible palindrome from given string `s`. 

    Parameters
    ----------
    s : str
        String containing both uppercase and/or lowercase letters.

    Constrains
    ----------
    -> 1 <= s.length <= 2000
    -> s consists of lowercase and/or uppercase English letters only.

    """
    def longestPalindrome(self, s):
        """ Hashmap-Counter implementation.
        
        Time Complexity : O(N) - linear | N is length of string .
        Space Complxity : O(K) - linear | K is the different kinds of char in string `s`. 
        """
        longest_palindrome_length = 0
        char_counter = Counter(s)
        for freq in char_counter.values():
            if not freq%2 == 0:
                longest_palindrome_length -= 1
        if longest_palindrome_length == 0:
            return len(s)
        return len(s) + longest_palindrome_length + 1
        

s = input("Enter input string: ")
res = Solution().longestPalindrome(s)
print("--> result : ", res)