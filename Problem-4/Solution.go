func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	if (len(nums1) > len(nums2)) {
		return findMedianSortedArrays(nums2, nums1)
	}

	left, right := 0, len(nums1)
	totalNum := (len(nums1) + len(nums2) + 1) / 2
	var i, j int
	for (left < right) {
		i = left + (right - left + 1) / 2
		j = totalNum - i

		if (nums1[i - 1] > nums2[j]) {
			right = i - 1
		} else {
			left = i
		}
	}

	i = left + (right - left + 1) / 2
	j = totalNum - i

	var nums1LeftMax, nums1RightMin, nums2LeftMax, nums2RightMin int
	if (i == 0) {
		nums1LeftMax = math.MinInt64
	} else {
		nums1LeftMax = nums1[i - 1]
	}
	if (i == len(nums1)) {
		nums1RightMin = math.MaxInt64
	} else {
		nums1RightMin = nums1[i]
	}
	if (j == 0) {
		nums2LeftMax = math.MinInt64
	} else {
		nums2LeftMax = nums2[j - 1]
	}
	if (j == len(nums2)) {
		nums2RightMin = math.MaxInt64
	} else {
		nums2RightMin = nums2[j]
	}

	if ((len(nums1) + len(nums2)) % 2 == 1) {
		return float64(max(nums1LeftMax, nums2LeftMax)) 
	}
	return float64(max(nums1LeftMax, nums2LeftMax) + min(nums1RightMin, nums2RightMin)) / 2 
}