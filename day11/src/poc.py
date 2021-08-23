import numpy as np

seat = np.array([
    [0, 0, 0],
    [-1, 0, -1],
    [0, 1, 0]])


def is_occupied(X):
    return (X + 1) // 2


def is_a_seat(X):
    return (X >= 0) * 1


middle_seat_mask = np.array([
    [0, 0, 0],
    [0, 1, 0],
    [0, 0, 0]])
