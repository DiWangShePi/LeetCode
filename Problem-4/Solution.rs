impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }

        let n = nums1.len();
        let m = nums2.len();
        let total_num = (n + m + 1) / 2;
        let (mut left, mut right) = (0, n);
        let (mut i, mut j) = (0, 0);
        
        while left < right {
            i = left + (right - left + 1) / 2;
            j = total_num - i;

            if nums1[i - 1] > nums2[j] {
                right = i - 1;
            } else {
                left = i;
            }
        }
        i = left + (right - left + 1) / 2;
        j = total_num - i;
        let max_i32 = std::i32::MAX;
        let min_i32 = std::i32::MIN;
        
        let nums1_left_max;
        let nums1_right_min;
        let nums2_left_max;
        let nums2_right_min;

        if i == 0 {
            nums1_left_max = min_i32;
        } else {
            nums1_left_max = nums1[i - 1];
        }
        if i == n {
            nums1_right_min = max_i32;
        } else {
            nums1_right_min = nums1[i];
        }
        if j == 0 {
            nums2_left_max = min_i32;
        } else {
            nums2_left_max = nums2[j - 1];
        }
        if j == m {
            nums2_right_min = max_i32;
        } else {
            nums2_right_min = nums2[j];
        }

        if (n + m) % 2 == 1 {
            return f64::from(nums1_left_max.max(nums2_left_max));
        }
        return f64::from(nums1_left_max.max(nums2_left_max) + nums1_right_min.min(nums2_right_min)) / 2.0;
    }
}
