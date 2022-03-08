## Find Median from Data Stream

The **median** is the middle value in an ordered integer list. If the size of the list is even, there is no middle value and the median is the mean of the two middle values.

    - For example, for `arr = [2,3,4]`, the median is `3`.
    - For example, for `arr = [2,3]`, the median is `(2 + 3) / 2 = 2.5`.

Implement the MedianFinder class:

    - `MedianFinder()` initializes the `MedianFinder` object.
    - `void addNum(int num)` adds the integer `num` from the data stream to the data structure.
    - `double findMedian()` returns the median of all elements so far. Answers within `10^(-5)` of the actual answer will be accepted.

---------------
**Follow up**:

    - If all integer numbers from the stream are in the range [0, 100], how would you optimize your solution?
    - If 99% of all integer numbers from the stream are in the range [0, 100], how would you optimize your solution?

-----------------
https://leetcode.com/problems/find-median-from-data-stream/
