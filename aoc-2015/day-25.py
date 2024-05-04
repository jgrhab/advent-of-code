ROW, COL = 2981, 3075

X_1 = 20151125
A, Q = 252533, 33554393

# Denote the elements of the sequence by X(N).
# It is defined as
#       X(N) = A**(N-1) * X(1)  mod Q
#
# # Finding N:
#
# There is a bijection T: (R, C) -> N,
# where (R, C) denote the positions in the table. We have:
#   (1)     T(R, 1) = 1 + (R-1) * R / 2
#   (2)     T(R, 1) + k = T(R-k, 1+k), for 0 <= k < R

# Given (R,C), we write k = C-1, S = R+k so that (R,C) = (S-k, 1+k).
# This corresponds to a value
#       N = T(S-k, 1+k)
#         = T(S, 1) + k
#         = (R+C-2) * (R+C-1) / 2 + C


def flatten_index(row: int, col: int) -> int:
    """Converts a table position (R, C) into a sequence index N."""
    return int((row + col - 1) * (row + col - 2) / 2) + col


target_ind = flatten_index(ROW, COL)


# Compute X(N)

x = X_1
for k in range(target_ind - 1):
    x *= A
    x %= Q


print(x)
