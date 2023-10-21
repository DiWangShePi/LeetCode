using namespace std;

#include <iostream>
#include <limits>
#include <vector>

class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
		if (nums1.size() > nums2.size()) {
			return findMedianSortedArrays(nums2, nums1);
		}

		int n = nums1.size(), m = nums2.size();

		int totaNum = (n + m + 1) / 2;
		int left = 0, right = n;
		int i = 0, j = 0;
		while(left < right) 
		{
			i = left + (right - left + 1) / 2;
			j = totaNum - i;

			if (nums1[i - 1] > nums2[j])
			{	// Next Round: [left, i - 1]
				right = i - 1;
			}
			else
			{   // Next Round: [i, right] 
				left = i;
			}
		}

		int maxIntValue = std::numeric_limits<int>::max();
		int minIntValue = std::numeric_limits<int>::min();
		
		i = left + (right - left + 1) / 2;
		j = totaNum - i;

		int nums1LeftMax = (i == 0) ? minIntValue : nums1[i - 1];
		int nums1RightMin = (i == n) ? maxIntValue : nums1[i];
		int nums2LeftMax = (j == 0) ? minIntValue : nums2[j - 1];
		int nums2RightMin = (j == m) ? maxIntValue : nums2[j];


		if ((n + m) % 2 == 1) 
		{
			return max(nums1LeftMax, nums2LeftMax);
		}
		return (double) ((max(nums1LeftMax, nums2LeftMax) + min(nums1RightMin, nums2RightMin))) / 2; 
    }
};


int main(){
    // Initialize two vectors to test the findMedianSortedArrays function
    vector<int> nums1 = {1, 2};
    vector<int> nums2 = {3, 4, 5};

    Solution solution;
    double median = solution.findMedianSortedArrays(nums1, nums2);

    cout << "Median: " << median << endl;

	return 0;
}

