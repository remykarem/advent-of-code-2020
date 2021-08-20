# O(n) space complexity
import numpy as np

nums = []
with open("input.txt", "r") as f:
    for line in f:
        nums.append(int(line.strip()))

nums.sort()
nums.append(nums[-1]+3)
nums2 = nums.copy()
nums2[-1] = 0

x = np.array(nums)
y = np.roll(np.array(nums2), 1)

differences, counts = np.unique(x - y, return_counts=True)
counts_diff1, counts_diff2_ = counts
counts_diff2 = counts_diff2_ + 1 # correction

print(counts_diff1*counts_diff2)
