from itertools import product
from collections import namedtuple

EqStats = namedtuple("Stats", ["cost", "damage", "armor"])
CharStats = namedtuple("CharStats", ["hp", "damage", "armor"])

BOSS = CharStats(hp=109, damage=8, armor=2)
PLAYER = CharStats(hp=100, damage=0, armor=0)

weapons = {
    "Dagger": EqStats(8, 4, 0),
    "Shortsword": EqStats(10, 5, 0),
    "Warhammer": EqStats(25, 6, 0),
    "Longsword": EqStats(40, 7, 0),
    "Greataxe": EqStats(74, 8, 0),
}

armors = {
    "None": EqStats(0, 0, 0),
    "Leather": EqStats(13, 0, 1),
    "Chainmail": EqStats(31, 0, 2),
    "Splintmail": EqStats(53, 0, 3),
    "Bandedmail": EqStats(75, 0, 4),
    "Platemail": EqStats(102, 0, 5),
}

rings = {
    "None 1": EqStats(0, 0, 0),
    "None 2": EqStats(0, 0, 0),
    "Damage +1": EqStats(25, 1, 0),
    "Damage +2": EqStats(50, 2, 0),
    "Damage +3": EqStats(100, 3, 0),
    "Defense +1": EqStats(20, 0, 1),
    "Defense +2": EqStats(40, 0, 2),
    "Defense +3": EqStats(80, 0, 3),
}


def sum_eqstats(*args: EqStats) -> EqStats:
    return EqStats(*[sum(x) for x in zip(*args)])


# Generate all combinations of equipment
eqs = []
for key_w, key_a in product(weapons, armors):
    for key_r1, key_r2 in ((k1, k2) for k1, k2 in product(rings, rings) if k1 < k2):
        eqs.append(
            sum_eqstats(weapons[key_w], armors[key_a], rings[key_r1], rings[key_r2])
        )

eqs.sort(key=lambda x: x.cost)


def player_wins(eq: EqStats) -> bool:
    """Returns True if the Player wins."""

    boss_hp, player_hp = BOSS.hp, PLAYER.hp
    player_damage = PLAYER.damage + eq.damage
    player_armor = PLAYER.armor + eq.armor

    while player_hp > 0 and boss_hp > 0:
        boss_hp -= max(1, player_damage - BOSS.armor)
        player_hp -= max(1, BOSS.damage - player_armor)

    return boss_hp <= 0


# -- Part One -- #

eqs_win = [eq for eq in eqs if player_wins(eq)]
print(eqs_win[0].cost)


# -- Part Two -- #

eqs_loss = [eq for eq in eqs if not player_wins(eq)]
print(eqs_loss[-1].cost)
