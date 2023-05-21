class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        res = []
        nums.sort()
        for l_index in range(len(nums) - 2):
            if l_index > 0 and nums[l_index] == nums[l_index - 1]: # skip duplicates
                continue 
            m_index = l_index + 1
            r_index = len(nums) - 1
            while m_index < r_index:
                temp = nums[l_index] + nums[m_index] + nums[r_index]
                if temp > 0:
                    r_index -= 1
                elif temp < 0:
                    m_index += 1
                else: # 0 = nums[l_index] + nums[m_index] + nums[r_index]
                    res.append([nums[l_index], nums[m_index], nums[r_index]])
                    # don't increment left ptr yet, keep pinching
                    while m_index < r_index and nums[m_index] == nums[m_index + 1]:
                        m_index += 1
                    while m_index < r_index and nums[r_index] == nums[r_index - 1]:
                        r_index -= 1

                    r_index -= 1
                    m_index += 1
        return res
