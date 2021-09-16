def find_spoken_number(starting: list, target_turn: int):
    spoken = {number: turn+1 for turn, number in enumerate(starting[:-1])}

    prev_spoken = starting[-1]

    turn = len(starting)+1
    while turn <= target_turn:

        if prev_spoken not in spoken:
            new = 0
        else:
            new = (turn-1) - spoken[prev_spoken]

        spoken[prev_spoken] = turn-1
        prev_spoken = new
        turn += 1

    return new


part1 = find_spoken_number([6, 13, 1, 15, 2, 0], 2020)
part2 = find_spoken_number([6, 13, 1, 15, 2, 0], 30000000)
