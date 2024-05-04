from collections import namedtuple
from collections.abc import Iterator
from dataclasses import dataclass
from itertools import product

Spell = namedtuple("Spell", ["cost", "damage", "heal", "armor", "mana", "duration"])

SPELLS = {
    "MM": Spell(cost=53, damage=4, heal=0, armor=0, mana=0, duration=1),
    "D": Spell(cost=73, damage=2, heal=2, armor=0, mana=0, duration=1),
    "S": Spell(cost=113, damage=0, heal=0, armor=7, mana=0, duration=6),
    "P": Spell(cost=173, damage=3, heal=0, armor=0, mana=0, duration=6),
    "R": Spell(cost=229, damage=0, heal=0, armor=0, mana=101, duration=5),
}

BOSS_HP = 58
BOSS_DAMAGE = 9
PLAYER_HP = 50
PLAYER_MANA = 500


@dataclass
class State:
    player_hp: int
    boss_hp: int
    mana: int
    armor: int
    active: dict[str, int]
    spent: int

    def __init__(self, player_hp=PLAYER_HP, boss_hp=BOSS_HP, mana=PLAYER_MANA, armor=0):
        self.player_hp = player_hp
        self.boss_hp = boss_hp
        self.mana = mana
        self.armor = armor
        self.active = {spell: 0 for spell in SPELLS}
        self.spent = 0

    def boss_attacks(self):
        self.player_hp -= max(1, BOSS_DAMAGE - self.armor)

    def apply_effect(self, spell: str):
        self.player_hp += SPELLS[spell].heal
        self.boss_hp -= SPELLS[spell].damage
        self.mana += SPELLS[spell].mana
        self.armor += SPELLS[spell].armor

    def tick(self):
        self.armor = 0

        for eff in (eff for eff, dur in self.active.items() if dur > 0):
            self.apply_effect(eff)
            self.active[eff] -= 1


def play_seqence(seq: tuple[str, ...], hard_mode_damage=0) -> State:
    state = State(player_hp=PLAYER_HP, boss_hp=BOSS_HP, mana=PLAYER_MANA, armor=0)

    for spell in seq:
        # Player turn
        state.player_hp -= hard_mode_damage

        if state.player_hp <= 0:
            return state

        state.mana -= SPELLS[spell].cost
        state.spent += SPELLS[spell].cost

        if state.mana < 0:
            return state

        state.tick()
        state.active[spell] += SPELLS[spell].duration

        # Boss turn
        state.tick()
        state.boss_attacks()

        if state.player_hp <= 0 or state.boss_hp <= 0:
            return state

    return state


def generate_sequences(turns: int) -> Iterator[tuple[str, ...]]:
    for seq in product(*((SPELLS.keys(),) * turns)):
        for ind, spell in enumerate(seq):
            if spell in seq[ind + 1 : ind + int(SPELLS[spell].duration / 2)]:
                continue

        yield seq


TURNS = 9  # smallest number that gives a win, found by trying
seqs = generate_sequences(TURNS)


# -- Part One -- #

min_spent = 1_000_000

for seq in seqs:
    final_state = play_seqence(seq)
    if final_state.mana >= 0 and final_state.boss_hp <= 0:
        min_spent = min(min_spent, final_state.spent)


print(min_spent)


# -- Part Two -- #

min_spent = 1_000_000

for seq in seqs:
    final_state = play_seqence(seq, hard_mode_damage=1)
    if final_state.mana >= 0 and final_state.boss_hp <= 0:
        min_spent = min(min_spent, final_state.spent)


print(min_spent)
