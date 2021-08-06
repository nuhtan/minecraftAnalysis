import sys
import csv
from typing import List

from anvil.region import Region

from mining import Direction, simpleBlock, veinExpansionIterable
from techniques import *

VALID_BLOCKS = {
    'iron_ore': 'iron',
    'deepslate_iron_ore': 'iron',
    'gold_ore': 'gold',
    'deepslate_gold_ore': 'gold',
    'diamond_ore': 'diamonds',
    'deepslate_diamond_ore': 'diamonds',
    'copper_ore': 'copper',
    'deepslate_copper_ore': 'copper',
    'redstone_ore': 'redstone',
    'deepslate_redstone_ore': 'redstone',
    'lapis_ore': 'lapis',
    'deepslate_lapis_ore': 'lapis',
    'coal_ore': 'coal',
    'deepslate_coal_ore': 'coal',
    'emerald_ore': 'emeralds',
    'deepslate_emerald_ore': 'emeralds'
}

def simulate(file: str, y: int, technique: str) -> dict[str, int]:
    """
    Runs a simulated mining technique at a specified y level
    
    Parameters
    ----------
    file
        The name of a region file in the regions directory
    y
        The y coordinate the the simulation should occur at
    technique
        The mining technique being simulated

    Returns
    -------
    A dictionary with the counts for various metrics
    """
    region = Region.from_file('regions/{}'.format(file))
    if technique == "basic":
        (total, mined) = standardBranchMining(region, Direction.SOUTH, tuple((255, y, 255)), 16, 160, 5)
    else:
        (total, mined) = branchWithPokeHoles(region, Direction.SOUTH, tuple((255, y, 255)), 10, 25, 5, 12)
    lava = []
    ores: List[simpleBlock] = []
    for block in total:
        if block.block.id == "lava" or block.block.id == "flowing_lava":
            lava.append(block)
        elif VALID_BLOCKS.get(block.block.id, None) is not None:
            ores.append(block)

    fullOres = []

    for y in range(len(ores)):
        expanded = veinExpansionIterable(region, ores[y].x, ores[y].y, ores[y].z)
        fullOres += expanded

    trimmed = []

    for ore in fullOres:
        found = False
        for comp in trimmed:
            if ore.x == comp.x and ore.y == comp.y and ore.z == comp.z:
                found = True
        if not found:
            trimmed.append(ore)

    results = {
        "coal": 0,
        "copper": 0,
        "iron": 0,
        "lapis": 0,
        "redstone": 0,
        "gold": 0,
        "emeralds": 0,
        "diamonds": 0,
        "lava": len(lava),
        "mined": mined
    }

    for ore in trimmed:
        oreType = VALID_BLOCKS.get(ore.block.id)
        results[oreType] += 1

    return results

def test(file, technique):
    """
    Runs a specified mining technique on a passed in region file and stores the results
    
    Parameters
    ----------
    file
        The region file that the simulation will run in, should be located in the regions directory
    technique
        The mining technique to simulate, currently either basic or poke
    """
    with open ('results/results-{}-{}.csv'.format(file, technique), mode='w', newline='') as csvFile:
        csvWriter = csv.writer(csvFile, delimiter=',')
        csvWriter.writerow(['y', 'blocks mined', 'lava', 'coal', 'copper', 'iron', 'lapis', 'redstone', 'gold', 'emeralds', 'diamonds'])
    for y in range(63, -60, -1):
        results = simulate(file, y, technique)
        with open ('results/results-{}-{}.csv'.format(file, technique), mode='a', newline='') as csvFile:
            csvWriter = csv.writer(csvFile, delimiter=',')
            csvWriter.writerow([y, results['mined'], results['lava'], results['coal'], results['copper'], results['iron'], results['lapis'], results['redstone'], results['gold'], results['emeralds'], results['diamonds']])
            print("Completed - File: {}, Y: {}".format(file, y))

def main():
    if len(sys.argv) == 3:
        test(sys.argv[1], sys.argv[2])
    else:
        print("Please run the program as 'python program.py filename basic'")

if __name__ == "__main__":
    main()