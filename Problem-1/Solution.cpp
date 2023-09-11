using namespace std;

#include <vector>
#include <iostream>

class Solution
{
public:
    vector<int> twoSum(vector<int> &nums, int target)
    {
        map<int, int> hashMap;
        int currentNum = 0;

        int n = nums.size();
        for (int i = 0; i < n; i++)
        {
            currentNum = nums[i];

            if (hashMap.count(currentNum) > 0)
            {
                return {hashMap[currentNum], i};
            }
            else
            {
                hashMap[target - currentNum] = i;
            }
        }
        return {};
    }
};

int main()
{
    vector<int> nums = {2, 7, 11, 15};
    int target = 9;

    // 创建 Solution 类的实例
    Solution solution;
    vector<int> result = solution.twoSum(nums, target);
    if (!result.empty())
    {
        cout << "Indices: " << result[0] << ", " << result[1] << endl;
    }
    else
    {
        cout << "No solution found." << endl;
    }
    return 0;
}