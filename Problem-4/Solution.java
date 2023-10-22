class Solution {
    public double findMedianSortedArrays(int[] nums1, int[] nums2) {
        if (nums1.length > nums2.length) {
            return findMedianSortedArrays(nums2, nums1);
        }

        int n = nums1.length, m = nums2.length;
        int left = 0, right = nums1.length;
        int totalNum = (n + m + 1) / 2;
        int i = 0, j = 0;
        while (left < right) {
            i = left + (right - left + 1) / 2;
            j = totalNum - i;

            if (nums1[i - 1] > nums2[j]) {
                right = i - 1;
            } else {
                left = i;
            }
        }

        i = left + (right - left + 1) / 2;
        j = totalNum - i;

        int nums1LeftMax = (i == 0) ? Integer.MIN_VALUE : nums1[i - 1];
        int nums1RightMin = (i == n) ? Integer.MAX_VALUE : nums1[i];
        int nums2LeftMax = (j == 0) ? Integer.MIN_VALUE : nums2[j - 1];
        int nums2RightMin = (j == m) ? Integer.MAX_VALUE : nums2[j];

        if ((n + m) % 2 == 1) {
            return Math.max(nums1LeftMax, nums2LeftMax);
        }
        return (double) (Math.max(nums1LeftMax, nums2LeftMax) + Math.min(nums1RightMin, nums2RightMin)) / 2;
    }
}