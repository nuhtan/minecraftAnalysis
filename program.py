from enum import Enum
from os import error
import sys
from typing import List
from anvil import *
from collections import deque
import csv

# All of the ores that we care about
valid = {
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

class simpleBlock:
    """A helper class that should probably be removed in favor of just using the Block class that this wraps."""
    def __init__(self, coords: tuple((int, int, int)), block: Block):
        self.x = coords[0]
        self.y = coords[1]
        self.z = coords[2]
        self.coords = coords
        self.block = block
    
    def __eq__(self, other) -> bool:
        return self.x == other.x and self.y == other.y and self.z == other.z

class Direction(Enum):
    """Self explanatory, only north, south, east, and west."""
    NORTH = 0,
    SOUTH = 1,
    EAST = 2,
    WEST = 3

def standardBranchMining(region: Region, baseDirection: Direction, startingCoords: tuple((int, int, int)), numberOfBranchPairs: int, branchLength: int, branchSpacing: int):
    """"""
    if branchSpacing < 2:
        raise error("Branch spacing for branch mining should be at least 2 to avoid duplicates")
        
    def corridorExpansion(coords: tuple((int, int, int))):
        blocks = []
        
        # duplicate handeling
        # This is the block that corresponds with the branches
        for y in range(-1, 3):
            blocks.append(simpleBlock(tuple((coords[0], coords[1] + y, coords[2])), getBlock(region, tuple((coords[0], coords[1] + y, coords[2])))))
        # The block after the first one
        for y in range(-1, 3):
            blocks.append(simpleBlock(dirCoords(baseDirection, tuple((coords[0], coords[1] + y, coords[2]))), getBlock(region, dirCoords(baseDirection, tuple((coords[0], coords[1] + y, coords[2]))))))
        # Final block of the corridor
        for y in range(-1, 3):
            blocks.append(simpleBlock(dirCoords(baseDirection, tuple((coords[0], coords[1] + y, coords[2])), branchSpacing), getBlock(region, dirCoords(baseDirection, tuple((coords[0], coords[1] + y, coords[2])), branchSpacing))))
        
        for n in range(2, branchSpacing):
            blocks += twoByOne(region, baseDirection, dirCoords(baseDirection, coords, n))

        # print("Corridor completed with {} blocks exposed".format(len(blocks)))
        return tuple((blocks, 2 + (2 * branchSpacing)))

    def branch(direction: Direction, coords: tuple((int, int, int))):
        blocks = []
        mined = 0
        for n in range(branchLength):
            blocks += twoByOne(region, direction, dirCoords(direction, coords, n))
            mined += 2

        blocks += twoByOneEnd(region, direction, dirCoords(direction, coords, branchLength - 1))
        # print("Branch completed with {} blocks exposed and {} mined.".format(len(blocks), mined))
        return tuple((blocks, mined))

    sumBlocks = []
    mined = 0
    branchDirection1, branchDirection2 = (Direction.NORTH, Direction.SOUTH) if baseDirection is Direction.EAST or baseDirection is Direction.WEST else Direction.EAST, Direction.WEST

    (tempBlocks, tempmined) = branch(branchDirection1, dirCoords(branchDirection1, startingCoords))
    sumBlocks += tempBlocks
    mined += tempmined
    (tempBlocks, tempmined) = branch(branchDirection2, dirCoords(branchDirection1, startingCoords))
    sumBlocks += tempBlocks
    mined += tempmined

    for n in range(numberOfBranchPairs - 1):
        (tempBlocks, tempmined) = corridorExpansion(dirCoords(baseDirection, startingCoords, n * 13))
        sumBlocks += tempBlocks
        mined += tempmined
        (tempBlocks, tempmined) = branch(branchDirection1, dirCoords(branchDirection1, startingCoords, n * 13))
        sumBlocks += tempBlocks
        mined += tempmined
        (tempBlocks, tempmined) = branch(branchDirection2, dirCoords(branchDirection1, startingCoords, n * 13))
        sumBlocks += tempBlocks
        mined += tempmined

    return tuple((sumBlocks, mined))

def branchWithPokeHoles(region: Region, baseDirection: Direction, startingCoords: tuple((int, int, int)), numberOfBranchPairs: int, pokesPerBranch: int, pokeSpacing: int, branchSpacing: int):
    if branchSpacing < 10:
        raise error("Branch spacing for branch mining with poke holes should be at least 10 to avoid duplicates")
    if pokeSpacing < 2:
        raise error("Poke spacing should be at least 2 to avoid duplicates")

    def corridorExpansion(coords: tuple((int, int, int))):
        blocks = []
        
        # duplicate handeling
        # This is the block that corresponds with the branches
        for y in range(-1, 3):
            blocks.append(simpleBlock(tuple((coords[0], coords[1] + y, coords[2])), getBlock(region, tuple((coords[0], coords[1] + y, coords[2])))))
        # The block after the first one
        for y in range(-1, 3):
            blocks.append(simpleBlock(dirCoords(baseDirection, tuple((coords[0], coords[1] + y, coords[2])), 1), getBlock(region, dirCoords(baseDirection, tuple((coords[0], coords[1] + y, coords[2]))))))
        # Final block of the corridor
        for y in range(-1, 3):
            blocks.append(simpleBlock(dirCoords(baseDirection, tuple((coords[0], coords[1] + y, coords[2])), branchSpacing), getBlock(region, dirCoords(baseDirection, tuple((coords[0], coords[1] + y, coords[2])), branchSpacing))))
        
        for n in range(2, 12):
            blocks += twoByOne(region, baseDirection, dirCoords(baseDirection, coords, n))

        # print("Corridor completed with {} blocks exposed".format(len(blocks)))
        return tuple((blocks, 2 + (2 * branchSpacing)))

    def branch(direction: Direction, coords: tuple((int, int, int))):
        blocks = []
        mined = 0
        for n in range(pokesPerBranch):
            coords = dirCoords(direction, coords, n * pokeSpacing) # Offset for the starting postion of each poke section
            # Base tunnel of branch
            for n in range(pokeSpacing):
                blocks += twoByOne(region, direction, dirCoords(direction, coords, n))

            # Poke start
            pokeCoords = list(coords)
            pokeCoords[1] += 1 # Raise the y by 1 for the pokes
            pokeCoords = dirCoords(direction, tuple(pokeCoords), pokeSpacing)
            # Gets directions perpendicular to the base tunnel direction
            if direction == Direction.EAST or direction == Direction.WEST:
                pokeDirection1, pokeDirection2 = Direction.NORTH, Direction.SOUTH  
            else:
                pokeDirection1, pokeDirection2 = Direction.EAST, Direction.WEST
            
            # The first side poke
            blocks += pokeStart(region, pokeDirection1, dirCoords(pokeDirection1, pokeCoords))
            blocks += oneByOne(region, pokeDirection1, dirCoords(pokeDirection1, pokeCoords, 2))
            blocks += oneByOne(region, pokeDirection1, dirCoords(pokeDirection1, pokeCoords, 3))
            blocks += oneByOne(region, pokeDirection1, dirCoords(pokeDirection1, pokeCoords, 4))
            blocks += oneByOneEnd(region, pokeDirection1, dirCoords(pokeDirection1, pokeCoords, 5))

            # Second side poke
            blocks += pokeStart(region, pokeDirection2, dirCoords(pokeDirection2, pokeCoords))
            blocks += oneByOne(region, pokeDirection2, dirCoords(pokeDirection2, pokeCoords, 2))
            blocks += oneByOne(region, pokeDirection2, dirCoords(pokeDirection2, pokeCoords, 3))
            blocks += oneByOne(region, pokeDirection2, dirCoords(pokeDirection2, pokeCoords, 4))
            blocks += oneByOneEnd(region, pokeDirection2, dirCoords(pokeDirection2, pokeCoords, 5))
            mined += 10 + (2 * pokeSpacing)

        blocks += twoByOneEnd(region, direction, dirCoords(direction, coords, (pokeSpacing * pokesPerBranch) - 1))
        # print("Branch completed with {} blocks exposed and {} mined.".format(len(blocks), mined))
        return tuple((blocks, mined))

    sumBlocks = []
    mined = 0
    branchDirection1, branchDirection2 = (Direction.NORTH, Direction.SOUTH) if baseDirection is Direction.EAST or baseDirection is Direction.WEST else Direction.EAST, Direction.WEST

    (tempBlocks, tempmined) = branch(branchDirection1, dirCoords(branchDirection1, startingCoords))
    sumBlocks += tempBlocks
    mined += tempmined
    (tempBlocks, tempmined) = branch(branchDirection2, dirCoords(branchDirection1, startingCoords))
    sumBlocks += tempBlocks
    mined += tempmined

    for n in range(numberOfBranchPairs - 1):
        (tempBlocks, tempmined) = corridorExpansion(dirCoords(baseDirection, startingCoords, n * 13))
        sumBlocks += tempBlocks
        mined += tempmined
        (tempBlocks, tempmined) = branch(branchDirection1, dirCoords(branchDirection1, startingCoords, n * 13))
        sumBlocks += tempBlocks
        mined += tempmined
        (tempBlocks, tempmined) = branch(branchDirection2, dirCoords(branchDirection1, startingCoords, n * 13))
        sumBlocks += tempBlocks
        mined += tempmined

    return tuple((sumBlocks, mined))
    

def dirCoords(direction: Direction, coords: tuple((int, int, int)), amount: int=1):
    temp = list(coords)
    if direction == Direction.NORTH:
        temp[2] -= amount
    if direction == Direction.SOUTH:
        temp[2] += amount
    if direction == Direction.EAST:
        temp[0] += amount
    if direction == Direction.WEST:
        temp[0] -= amount
    return tuple(temp)
        

def twoByOne(region: Region, direction: Direction, coords: tuple((int, int, int))):
    # [ ][x][ ]
    # [x][x][x]
    # [x][x][x]
    # [ ][x][ ]
    blocks = []
    xrange = [0]
    zrange = [0]
    if direction == Direction.NORTH or direction == Direction.SOUTH:
        xrange = range(-1, 2)
    if direction == Direction.EAST or direction == Direction.WEST:
        zrange = range(-1, 2)
    for x in xrange:
        for y in range(0, 2): # The player character is 2 blocks tall and the y coord represents the feet
            for z in zrange:
                blocks.append(simpleBlock(tuple((coords[0] + x, coords[1] + y, coords[2] + z)), getBlock(region, tuple((coords[0] + x, coords[1] + y, coords[2] + z)))))
    blocks.append(simpleBlock(tuple((coords[0], coords[1] - 1, coords[2])), getBlock(region, tuple((coords[0], coords[1] - 1, coords[2])))))
    blocks.append(simpleBlock(tuple((coords[0], coords[1] + 2, coords[2])), getBlock(region, tuple((coords[0], coords[1] + 2, coords[2])))))
    return blocks

# Maybe this should be made into a more generalized function
def oneByOne(region: Region, direction: Direction, coords: tuple((int, int, int))):
    # [ ][x][ ]
    # [x][x][x]
    # [ ][x][ ]
    blocks = []
    xrange = [0]
    zrange = [0]
    if direction == Direction.NORTH or direction == Direction.SOUTH:
        xrange = range(-1, 2)
    if direction == Direction.EAST or direction == Direction.WEST:
        zrange = range(-1, 2)
    for x in xrange:
        for y in range(0, 1):
            for z in zrange:
                blocks.append(simpleBlock(tuple((coords[0] + x, coords[1] + y, coords[2] + z)), getBlock(region, tuple((coords[0] + x, coords[1] + y, coords[2] + z)))))
    blocks.append(simpleBlock(tuple((coords[0], coords[1] - 1, coords[2])), getBlock(region, tuple((coords[0], coords[1] - 1, coords[2])))))
    blocks.append(simpleBlock(tuple((coords[0], coords[1] + 1, coords[2])), getBlock(region, tuple((coords[0], coords[1] + 1, coords[2])))))
    return blocks

def oneByOneEnd(region: Region, direction: Direction, coords: tuple((int, int, int))):
    blocks = oneByOne(region, direction, coords)
    if direction == Direction.NORTH:
        blocks.append(simpleBlock(tuple((coords[0], coords[1], coords[2] - 1)), getBlock(region, tuple((coords[0], coords[1], coords[2] - 1)))))
    elif direction == Direction.SOUTH:
        blocks.append(simpleBlock(tuple((coords[0], coords[1], coords[2] + 1)), getBlock(region, tuple((coords[0], coords[1], coords[2] + 1)))))
    elif direction == Direction.EAST:
        blocks.append(simpleBlock(tuple((coords[0] + 1, coords[1], coords[2])), getBlock(region, tuple((coords[0] + 1, coords[1], coords[2])))))
    elif direction == Direction.WEST:
        blocks.append(simpleBlock(tuple((coords[0] - 1, coords[1], coords[2])), getBlock(region, tuple((coords[0] - 1, coords[1], coords[2])))))
    return blocks

def twoByOneEnd(region: Region, direction: Direction, coords: tuple((int, int, int))):
    blocks = []
    if direction == Direction.NORTH:
        blocks.append(simpleBlock(tuple((coords[0], coords[1], coords[2] - 1)), getBlock(region, tuple((coords[0], coords[1], coords[2] - 1)))))
        blocks.append(simpleBlock(tuple((coords[0], coords[1] + 1, coords[2] - 1)), getBlock(region, tuple((coords[0], coords[1] + 1, coords[2] - 1)))))
    elif direction == Direction.SOUTH:
        blocks.append(simpleBlock(tuple((coords[0], coords[1], coords[2] + 1)), getBlock(region, tuple((coords[0], coords[1], coords[2] + 1)))))
        blocks.append(simpleBlock(tuple((coords[0], coords[1] + 1, coords[2] + 1)), getBlock(region, tuple((coords[0], coords[1] + 1, coords[2] + 1)))))
    elif direction == Direction.EAST:
        blocks.append(
            simpleBlock(
                tuple(
                    (coords[0] + 1, coords[1], coords[2])
                ), getBlock(
                    region, tuple(
                    (coords[0] + 1, coords[1], coords[2])
                    )
                )
            )
        )
        blocks.append(simpleBlock(tuple((coords[0] + 1, coords[1] + 1, coords[2])), getBlock(region, tuple((coords[0] + 1, coords[1] + 1, coords[2])))))
    elif direction == Direction.WEST:
        blocks.append(simpleBlock(tuple((coords[0] - 1, coords[1], coords[2])), getBlock(region, tuple((coords[0] - 1, coords[1], coords[2])))))
        blocks.append(simpleBlock(tuple((coords[0] - 1, coords[1] + 1, coords[2])), getBlock(region, tuple((coords[0] - 1, coords[1] + 1, coords[2])))))
    return blocks

# For the start of a poke the only unique block that is added from it is the block above the starting block
def pokeStart(region: Region, direction: Direction, coords: tuple((int, int, int))):
    blocks = []
    blocks.append(simpleBlock(tuple((coords[0], coords[1] + 1, coords[2])), getBlock(region, tuple((coords[0], coords[1] + 1, coords[2])))))
    return blocks

# queue has a item, add all of the surrounding blocks that are not already expanded and are part of the target list
# remove the first item from the list and add it to the expanded list
# repeat until there are no more items in the queue
# return the expanded list
def veinExpansionIterable(region: Region, targetx: int, targety: int, targetz: int):
    deq = deque()
    deq.append(simpleBlock(tuple((targetx, targety, targetz)), getBlock(region, tuple((targetx, targety, targetz)))))
    expanded = []
    current = 1

    while len(deq) > 0:
        # if current % 100 == 0:
            # print("{} blocks in queue, {} in the expanded set.".format(len(deq), len(expanded)))
        block = deq.popleft()
        expanded.append(block)
        for x in range(-1, 2):
            for y in range(-1, 2):
                for z in range(-1, 2):
                    if not (x == 0 and y == 0 and z == 0):
                        adj = simpleBlock(tuple((block.x + x, block.y + y, block.z + z)), getBlock(region, tuple((block.x + x, block.y + y, block.z + z))))
                        if adj.block.id in valid:
                            found = False
                            for b in expanded:
                                if b.x == adj.x and b.y == adj.y and b.z == adj.z:
                                    found = True
                            for b in deq:
                                if b.x == adj.x and b.y == adj.y and b.z == adj.z:
                                    found = True
                            if not found:
                                deq.append(adj)
        current += 1
    
    return expanded



# This is a helper function to remove duplicates, this could likely be accomplished with a lambda function
def veinExpansionTrimming(found: list[simpleBlock]):
    sublist = []
    for i in range(len(found)):
        if found[i] not in sublist:
            sublist.append(found[i])
    return sublist

# For this project we will only be using the 0,0 region and thus we can only accept 
def getBlock(region: Region, coords: tuple((int, int, int))) -> Block:
    chunkx = coords[0] // 16
    chunkz = coords[2] // 16
    chunk = region.get_chunk(chunkx, chunkz)
    return chunk.get_block(coords[0] % 16, coords[1], coords[2] % 16)

def run(file, y, style):
    region = Region.from_file('regions/{}'.format(file))
    # print("Loaded file: {}".format(file))
    # print("--------------------")
    # print("Simulating mining:")
    if style == "basic":
        (total, mined) = standardBranchMining(region, Direction.SOUTH, tuple((255, y, 255)), 16, 160, 5)
    elif style == "poke":
        (total, mined) = branchWithPokeHoles(region, Direction.SOUTH, tuple((255, y, 255)), 10, 25, 5, 12)
    
    # print("Mining complete")
    # print("--------------------")

    # print("Seperating ores and lava:")
    lava = []
    ores: List[simpleBlock] = []
    for block in total:
        if block.block.id == "lava" or block.block.id == "flowing_lava":
            lava.append(block)
        elif valid.get(block.block.id, None) is not None:
            ores.append(block)
    # print("Seperating complete")
    # print("--------------------")

    # If there was a simple way to check if two position are 'connected' we could pre trim the list of ores to be expanded.

    # print("Expanding ores:")
    fullOres = []

    for y in range(len(ores)):
        expanded = veinExpansionIterable(region, ores[y].x, ores[y].y, ores[y].z)
        # print("expanded: {}/{} with {} found".format(y + 1, len(ores), len(expanded)))
        fullOres += expanded
    # print("Expansion complete")
    # print("--------------------")

    # print("Removing duplicates from expanded ores:")
    trimmed = []

    for ore in fullOres:
        found = False
        for comp in trimmed:
            if ore.x == comp.x and ore.y == comp.y and ore.z == comp.z:
                found = True
        if not found:
            trimmed.append(ore)
    # print("Duplicates removed")
    # print("--------------------")

    # print("{} blocks mined".format(mined))
    # print("{} blocks exposed".format(len(total)))
    # print("{} lava found".format(len(lava)))
    # print("{} unique ore found with simple expansion".format(len(trimmed)))
    # print("--------------------")

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
        oreType = valid.get(ore.block.id)
        results[oreType] += 1

    # print("{} coal found".format(results["coal"]))
    # print("{} copper found".format(results["copper"]))
    # print("{} iron found".format(results["iron"]))
    # print("{} lapis found".format(results["lapis"]))
    # print("{} redstone found".format(results["redstone"]))
    # print("{} gold found".format(results["gold"]))
    # print("{} emeralds found".format(results["emeralds"]))
    # print("{} diamonds found".format(results["diamonds"]))
    # print("Completed - File: {}, Y: {}".format(file, y))
    return results

def test(file, style):
    with open ('results/results-{}-{}.csv'.format(file, style), mode='w', newline='') as csvFile:
        csvWriter = csv.writer(csvFile, delimiter=',')
        csvWriter.writerow(['y', 'blocks mined', 'lava', 'coal', 'copper', 'iron', 'lapis', 'redstone', 'gold', 'emeralds', 'diamonds'])
    for y in range(63, -60, -1):
        results = run(file, y, style)
        with open ('results/results-{}-{}.csv'.format(file, style), mode='a', newline='') as csvFile:
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