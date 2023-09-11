using namespace std;

#include <vector>
#include <iostream>

class Solution
{
public:
    vector<int> twoSum(vector<int> &nums, int target)
    {
        vector<bool> existNum(target, false);
        vector<int> myVector(target, 0);

        vector<int> answer;
        int currentNum = 0;
        int position = 0;
        for (int i = 0; i < nums.size(); i++)
        {
            currentNum = nums[i];
            if (currentNum >= target)
            {
                continue;
            }

            position = target - currentNum;
            if (!existNum[position])
            {
                existNum[position] = true;
                myVector[position] = i;
            }
            else
            {
                answer.push_back(myVector[position]);
                answer.push_back(i);
                return answer;
            }
        }
        return answer;
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