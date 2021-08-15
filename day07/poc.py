table = {

    "blue": 0,
    "black": 0,

}

to_find = {

    "red": [(1, "white"), (2, "yellow")],
    "orange": [(3, "white"), (4, "yellow")],
    "white": [(1, "gold")],
    "yellow": [(2, "gold"),  (9, "faded")],
    "gold": [(1, "olive"), (2, "plum")],
    "olive": [(3, "blue"),  (4, "black")],
    "plum": [(5, "blue"),  (6, "black")],

}


def f(colour):
    """
        ----- child -----    ----- child -----
    colour: [(qty, child_colour), (qty, child_colour)]
    """

    if colour in table:
        return table[colour]

    count = 0

    for child in to_find[colour]:

        qty, child_colour = child

        child_count = f(child_colour)

        if child_count == 0:
            count += qty
        else:
            count += qty + qty * child_count

    # Update memoisation
    table[colour] = count

    # Update to_find
    del to_find[colour]

    return count


assert f("gold") == 126
