package twoSum

// This function takes an array of integers, and a target integer as arguments.
// It searches the array of ints for a unique pair of elements that sum to the
// target value. Then returns that pair as an array

// ~4ms solution, ~10x faster than nested for loop O(n^2) solution
// LOOK INTO SYNTAX ON LINE 13
func twoSum_4ms(nums []int, target int) []int {
	seen := map[int]int{}
	for i := range nums {
		if _, ok := seen[target-nums[i]]; ok {
			return []int{seen[target-nums[i]], i}
		}
		seen[nums[i]] = i
	}
	return []int{}
}
