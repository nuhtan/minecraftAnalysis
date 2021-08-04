from mining import *
from os import error

from anvil.region import Region


def standardBranchMining(region: Region, baseDirection: Direction, startingCoords: tuple[int, int, int], numberOfBranchPairs: int, branchLength: int, branchSpacing: int) -> tuple[list[simpleBlock], int]:
    """
    A simple mining technique that consists of a main corridor that has branches coming of it perpendicular to the main corridor.

    Parameters
    ----------
    region
        The region file that the simulation will occur in
    baseDirection
        The direction that the main corridor will be expanding
    startingCoords
        A tuple of the xyz coords that the main corridor will start at
    numberOfBranchPairs
        The number of branches that will stretch out perpendicular to the main corridor
    branchLength
        The length of each branch
    branchSpacing
        The number of blocks in between each pair of branches

    Returns
    -------
    A tuple consisting of all blocks that would be exposed from mining and the number of blocks that would have been mined to replicate the simulation in game.
    """
    if branchSpacing < 2:
        raise error("Branch spacing for branch mining should be at least 2 to avoid duplicates")
        
    def corridorExpansion(coords: tuple[int, int, int]) -> tuple[list[simpleBlock], int]:
        """
        Expands the central corridor based on the branch spacing of the parent function

        Parameters
        ----------
        coords
            The xyz position of where the expansion should start

        Returns
        -------
        A tuple consisting of a list of the blocks exposed and a count of the blocks that would need to be mined.
        """
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

        return tuple[blocks, 2 + (2 * branchSpacing)]

    def branch(direction: Direction, coords: tuple[int, int, int]) -> tuple[list[simpleBlock], int]:
        """
        Simulates mining a branch in a direction

        Parameters
        ----------
        direction
            The direction that the branch should expand in
        coords
            The xyz position that the branch should start at

        Returns
        -------
        A tuple consisting of a list of the blocks exposed and a count of the blocks that would need to be mined.
        """
        blocks = []
        mined = 0
        for n in range(branchLength):
            blocks += twoByOne(region, direction, dirCoords(direction, coords, n))
            mined += 2

        blocks += twoByOneEnd(region, direction, dirCoords(direction, coords, branchLength - 1))
        return tuple[blocks, mined]

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

    return tuple[sumBlocks, mined]

def branchWithPokeHoles(region: Region, baseDirection: Direction, startingCoords: tuple[int, int, int], numberOfBranchPairs: int, pokesPerBranch: int, pokeSpacing: int, branchSpacing: int) -> tuple[list[simpleBlock], int]:
    """
    
    """
    if branchSpacing < 10:
        raise error("Branch spacing for branch mining with poke holes should be at least 10 to avoid duplicates")
    if pokeSpacing < 2:
        raise error("Poke spacing should be at least 2 to avoid duplicates")

    def corridorExpansion(coords: tuple[int, int, int]):
        """
        
        """
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

        return tuple((blocks, 2 + (2 * branchSpacing)))

    def branch(direction: Direction, coords: tuple[int, int, int]):
        """
        
        """
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
        return tuple[blocks, mined]

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

    return tuple[sumBlocks, mined]