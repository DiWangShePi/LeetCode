import java.util.HashMap

class Solution {
    public int[] twoSum(int[] nums, int target) {
        HashMap<Integer, Integer> hashMap = new HashMap<>();
        int[] result = new int[2];

        int currentNum = 0;
        for (int i = 0; i < nums.length; i++){
            currentNum = nums[i];

            if (hashMap.containsKey(currentNum)) {
                result[0] = hashMap.get(currentNum);
                result[1] = i;
                
                return result;
            }
            hashMap.put(target - currentNum, i);
        }
        return result;
    }
}