from typing import List

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        hashMap = {}

        for i, currentNum in enumerate(nums):
            if currentNum in hashMap:
                return [hashMap[currentNum], i]
            else:
                hashMap[target - currentNum] = i
        return []

if __name__ == "__main__":
    try_sol = Solution()
    answer = try_sol.twoSum([3, 10, 13, -3], 0)
    print(answer)
