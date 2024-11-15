The problem is similar to day-23 in that it contains loops in the code.
We would need to identify the loops to reduce the execution time of the program if we intend to test many input values.
However, identifying the loops makes it so that we can solve the problem by hand.

The initial values in the registers are {a: A, b: B, c: C, d: A}, where A is the value to determine and B and C are given by the initial `cpy` commands.\

The program contains the following loops:

cmd | loop  | loop  | loop  | loop  |
----|-------|-------|-------|-------|
0   |       |       |       |       |
1   |       |       |       |       |
2   |       |   4   |       |       |
3   |   1   |   4   |       |       |
4   |   1   |   4   |       |       |
5   |   1   |   4   |       |       |
6   |       |   4   |       |       |
7   |       |   4   |       |       |
8   |       |       |       |   7   |
9   |       |       |   6   |   7   |
10  |       |       |   6   |   7   |
11  |       |       |   6   |   7   |
12  |       |   5   |   6   |   7   |
13  |   2   |   5   |   6   |   7   |
14  |   2   |   5   |   6   |   7   |
15  |   2   |   5   |   6   |   7   |
16  |   2   |   5   |   6   |   7   |
17  |   2   |   5   |   6   |   7   |
18  |       |   5   |   6   |   7   |
19  |       |   5   |   6   |   7   |
20  |       |       |   6   |   7   |
21  |   3   |       |   6   |   7   |
22  |   3   |       |   6   |   7   |
23  |   3   |       |   6   |   7   |
24  |   3   |       |   6   |   7   |
25  |   3   |       |   6   |   7   |
26  |       |       |   6   |   7   |
27  |       |       |   6   |   7   |
28  |       |       |   6   |   7   |
29  |       |       |       |   7   |


These have the following effect:

- loop 1:
    - d = d + b
    - b = 0

- loop 2:  treat as part of loop 5

- loop 3:
    - b = b - c
    - c = 0

- loop 4:
    - d = d + bc
    - b = 0
    - c = 0

- loop 5: for one iteration:
    - a = a // 2
    - b = a % 2
    - c = 0
    - out a % 2
    - repeat while a > 0

- loop 5:
    - a = d
    - loop 6

So in order to output 0, 1, 0, 1, ..., we must enter loop 5 with a value a0 of a 
which satisfies:
a0 even, a1 := a0 // 2 odd, a2 := a1 // 2 even, a3 := a2 // 2 odd, ...
This sequences ends when we reach a = 0.
The ak must satisfy:
- a_2k = 2 a_2k+1
- a_2k+1 = 2 a_2k+2 + 1
Since we start with a = d (at the start of loop 7), this means that the starting value
(the value of d) must be in the sequence:
\[ d_{n+1} = 2 (2d_n + 1). \]

The first values of this sequence are 0, 2, 10, 42, 170, 682, 2730, ...
and since the value of d when entering loop 7 for the first time is determined
by the commands above, it can be computed and is seen to be d = A + BC.
Thus, all we have to do is find the smallest positive value of A for which
A + BC is in the sequence, with the values of B and C dtermined by the first commands.
