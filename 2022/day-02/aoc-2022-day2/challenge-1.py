# --- Day 2: Rock Paper Scissors --- The Elves begin to set up camp on the beach. To decide whose tent gets to be
# closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.
#
# Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each
# simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is
# selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same
# shape, the round instead ends in a draw.
#
# Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they
# say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock,
# B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.
#
# The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for
# Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.
#
# The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores
# for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper,
# and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw,
# and 6 if you won).
#
# Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get
# if you were to follow the strategy guide.
#
# For example, suppose you were given the following strategy guide:
#
# A Y
# B X
# C Z
# This strategy guide predicts and recommends the following:
#
# In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you
# with a score of 8 (2 because you chose Paper + 6 because you won). In the second round, your opponent will choose
# Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0). The third round
# is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6. In this example, if you were to
# follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
#
# What would your total score be if everything goes exactly according to your strategy guide?
ROCK = "Rock"
PAPER = "Paper"
SCISSORS = "Scissors"


def __calculate_score():
    file = open('input.txt', 'r')
    lines = file.readlines()

    total = 0

    for line in lines:
        total += __calculate_line_result(line)

    print("Total points: {}".format(total))


def __calculate_line_result(line):
    if len(line.strip()) > 0:
        print("Line: {}".format(line.strip()))
        opponent = __decode_to_rock_paper_scissors(line[0:1])
        me = __decode_to_rock_paper_scissors(line[2:3])

        print("Opponent's hand: {}".format(opponent))
        print("My hand: {}".format(me))

        result_points = __calculate_points_for_result(opponent, me)
        choice_point = __calculate_point_for_my_choice(me)
        points = result_points + choice_point
        print("Points: {}".format(points))
        print(' ')
        return points


def __decode_to_rock_paper_scissors(choice):
    if choice == 'A' or choice == 'X':
        return ROCK
    elif choice == 'B' or choice == 'Y':
        return PAPER
    elif choice == 'C' or choice == 'Z':
        return SCISSORS
    else:
        raise ValueError("Wrong input value: {}".format(choice))


def __calculate_point_for_my_choice(me):
    return 1 if me == ROCK else 2 if me == PAPER else 3


def __calculate_points_for_result(opponent, me):
    if (opponent == ROCK and me == PAPER) or (opponent == PAPER and me == SCISSORS) or (opponent == SCISSORS and me == ROCK):
        return 6
    elif opponent == me:
        return 3
    else:
        return 0


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    __calculate_score()
