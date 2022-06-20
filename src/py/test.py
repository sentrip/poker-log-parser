import pklpy
import json

example = """PokerStars Hand #33000: Hold'em No Limit ($50/$100) - 2019/07/11 09:10:00 ET
Table 'Pluribus Session 33' 6-max Seat #1 is the button
Seat 1: MrWhite ($10000 in chips)
MrWhite: posts small blind $50
*** HOLE CARDS ***
MrWhite: folds
*** SUMMARY ***
Total pot $50 | Rake 0
"""

if __name__ == "__main__":
    import pprint
    pprint.pprint(json.loads(pklpy.str_to_json(example)))
