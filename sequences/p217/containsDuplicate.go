package duplicate

// this function accepts an array of integers as an argument and returns
// whether any of it's element repeat (true if yes)

func containsDuplicate(nums []int) bool {
	intMap := map[int]string{}
	for _, n := range nums {
		if intMap[n] != "" {
			return true
		}
		intMap[n] = string(n)
	}
	return false
}
