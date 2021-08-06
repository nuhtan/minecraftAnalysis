from collections import deque
from enum import Enum

from anvil import Region

class simpleBlock:
    """
    An abstraction of the Block class from the anvil library. This class consists of the coordinates of a block along with the name.
    """
    def __init__(self, coords: tuple[int, int, int], block: str):
        """
        Parameters
        ----------
        coords
            The coordinates that the block occupies
        block
            The name of the block ie. 'gold_ore'
        """
        self.x = coords[0]
        self.y = coords[1]
        self.z = coords[2]
        self.coords = coords
        self.block = block
    
    def __eq__(self, other) -> bool:
        """Only checks that the coordinates are the same"""
        return self.x == other.x and self.y == other.y and self.z == other.z

class Direction(Enum):
    """Self explanatory, only north, south, east, and west."""
    NORTH = 0,
    SOUTH = 1,
    EAST = 2,
    WEST = 3

def dirCoords(direction: Direction, coords: tuple[int, int, int], amount: int=1) -> tuple[int, int, int]:
    temp = list(coords)
    if direction == Direction.NORTH:
        temp[2] -= amount
    elif direction == Direction.SOUTH:
        temp[2] += amount
    elif direction == Direction.EAST:
        temp[0] += amount
    elif direction == Direction.WEST:
        temp[0] -= amount
    return tuple(temp)

def twoByOne(region: Region, direction: Direction, coords: tuple[int, int, int]) -> list[simpleBlock]:
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
def veinExpansionIterable(region: Region, valid: dict[str, str],targetx: int, targety: int, targetz: int):
    deq = deque()
    deq.append(simpleBlock(tuple((targetx, targety, targetz)), getBlock(region, tuple((targetx, targety, targetz)))))
    expanded = []
    current = 1

    while len(deq) > 0:
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

def getBlock(region: Region, coords: tuple((int, int, int))) -> str:
    """
    Gets a block id from a region file using input coordinates.

    Parameters
    ----------
    region
        The region file to be searched
    coords
        The coordinates of the block

    Returns
    -------
    The name block id of the block
    """
    chunkx = coords[0] // 16
    chunkz = coords[2] // 16
    chunk = region.get_chunk(chunkx, chunkz)
    return chunk.get_block(coords[0] % 16, coords[1], coords[2] % 16).id