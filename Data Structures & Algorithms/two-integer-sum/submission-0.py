class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        seen = {}
        for i, u in enumerate(nums):
            complementary = target - u
            if complementary in seen:
                return [seen[complementary], i]
            else:
                seen[u] = i