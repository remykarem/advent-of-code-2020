# https://fangya.medium.com/chinese-remainder-theorem-with-python-a483de81fbb8
from functools import reduce


def chinese_remainder(n, a):
    sm = 0
    prod = reduce(lambda a, b: a*b, n)
    for n_i, a_i in zip(n, a):
        p = prod//n_i  # for large p*/
        sm += a_i * mul_inv(p, n_i)*p
    return sm % prod


def mul_inv(a, b):
    b0 = b
    x0, x1 = 0, 1
    if b == 1:
        return 1
    while a > 1:
        q = a // b
        a, b = b, a % b
        x0, x1 = x1 - q * x0, x0
    if x1 < 0:
        x1 += b0
    return x1


a = [0, -9, -13, -19, -32, -42, -48, -50, -67]
n = [19, 41, 37, 787, 13, 23, 29, 571, 17]

chinese_remainder(n, a)
