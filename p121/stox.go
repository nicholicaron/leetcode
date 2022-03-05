package stox

// Takes an array of stock prices, indexed by the day that price is available
// finds max difference (profit) between indexed values (prices on diff days)

func maxProfit(prices []int) int {
	maxPrice := 0
	minPrice := prices[0]
	maxProfit := 0

	for _, price := range prices {
		// assign price to maxPrice to ensure maxPrice does not remain at an
		// earlier index than minPrice
		// how to prevent minPrice from updating???? eg. case: prices = [2,4,1]
		// outputting 0 instead of 2 bc minPrice updates to 1
		if price < minPrice {
			minPrice = price
			maxPrice = price
		} else if price > maxPrice {
			maxPrice = price
		}
        newMaxProfit := maxPrice - minPrice
        if (newMaxProfit > maxProfit) {
            maxProfit = newMaxProfit
        }
		
        
	}

	if maxProfit < 0 {
		return 0
	} else {
		return maxProfit
	}
}
