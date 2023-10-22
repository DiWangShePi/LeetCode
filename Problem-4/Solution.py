import sys

class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        if len(nums1) > len(nums2) :
            return self.findMedianSortedArrays(nums2, nums1)
        
        left = 0
        right : int = len(nums1)
        totalNum : int = (len(nums1) + len(nums2) + 1) // 2
        while left < right :
            i : int = left + (right - left + 1) // 2
            j : int = totalNum - i
             
            if nums1[i - 1] > nums2[j] :
                right = i - 1
            else: 
                left = i
        i = left + (right - left + 1) // 2
        j = totalNum - i
        
        nums1LeftMax = (-sys.maxsize - 1) if i == 0 else nums1[i - 1]
        nums1RightMin = sys.maxsize if i == len(nums1) else nums1[i]
        nums2LeftMax = (-sys.maxsize - 1) if j == 0 else nums2[j - 1]
        nums2RightMin = sys.maxsize if j == len(nums2) else nums2[j]
        
        if (len(nums1) + len(nums2)) % 2 == 1 :
            return max(nums1LeftMax, nums2LeftMax)
        return (max(nums1LeftMax, nums2LeftMax) + min(nums1RightMin, nums2RightMin)) / 2