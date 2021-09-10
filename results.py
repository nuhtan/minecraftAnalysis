#%%
import os
from typing import Dict
from pandas.core.frame import DataFrame
import pandas
import seaborn as sns
import matplotlib.pyplot as plt

#%%
outDir = "results"
chunkDir = "chunk_data"
miningDir = "mining_data"


def chunk_data():
    files = []
    container: Dict[str, DataFrame] = {}

    for filename in os.listdir(chunkDir):
        with open('chunk_data/' + filename) as file:
            data = pandas.read_csv(file)
            files.append(data)
    print("Files added")

    combined = pandas.concat(files)
    print("Files combined")

    for col in combined.columns[3:]:
        container[col] = DataFrame(columns=['y', 'avgBlocksPerChunk'])
    print("Columns added")

    for y in range(-64, 320):
        y_df = combined[combined['y'] == y]
        for col in y_df.columns[3:]:
            container[col] = container[col].append({'y': y, 'avgBlocksPerChunk': y_df.mean()[col]}, ignore_index=True)
    print("Range looped")


    air = container["air"];
    air.to_csv("results/chunks_air_full_range.csv", index=False)
    for subset in container:
        blockData = container[subset]
        blockData[blockData['y'] <= 65].to_csv("results/" + subset + "_chunks.csv", index=False)

def chunk_graphs():
    sns.set_theme()
    for filename in os.listdir(outDir):
        if filename.count("chunks") > 0: # Only want the chunk data for this
            plt.figure(figsize=(35, 10))
            dataset: DataFrame = pandas.read_csv('results/' + filename)
            figure = sns.lineplot(data=dataset, x="y", y="avgBlocksPerChunk")
            if filename.count("full") > 0: # The full range needs slight changes
                figure.set_xlim([-64, 320])
                figure.set_ylim([0, 260])
                figure.set_title("Air Blocks Full Range")
                plt.savefig("graphical_results/chunks_air_full.png")
            else:
                figure.set_xlim([-64, 65])
                figure.set_title(filename.split("_")[0])
                plt.savefig("graphical_results/chunks_" + filename.split("_")[0] + ".png")

def techniqueData():
    t1 = []
    t2 = []
    container1: Dict[str, DataFrame] = {}
    container2: Dict[str, DataFrame] = {}

    for filename in os.listdir(miningDir):
        with open('mining_data/' + filename) as file:
            data = pandas.read_csv(file)
            if filename.count("branch") > 0:
                t1.append(data)
            else:
                t2.append(data)
    print("Files added")

    combined1 = pandas.concat(t1)
    combined2 = pandas.concat(t2)
    print("Files combined")

    for col in combined1.columns[3:]:
        container1[col] = DataFrame(columns=['y', 'blocksPerSimulation'])
        container2[col] = DataFrame(columns=['y', 'blocksPerSimulation'])
    print("Columns added")

    for y in range(-64, 65):
        y_df1 = combined1[combined1['y'] == y]
        y_df2 = combined2[combined2['y'] == y]
        for col in y_df1.columns[3:]:
            container1[col] = container1[col].append({'y': y, 'blocksPerSimulation': y_df1.mean()[col]}, ignore_index=True)
        for col in y_df2.columns[3:]:
            container2[col] = container2[col].append({'y': y, 'blocksPerSimulation': y_df2.mean()[col]}, ignore_index=True)

    for subset in container1:
        blockData = container1[subset]
        blockData.to_csv("results/" + subset + "_branch.csv", index=False)
    for subset in container2:
        blockData = container2[subset]
        blockData.to_csv("results/" + subset + "_poke.csv", index=False)

def technique_graphs():
    sns.set_theme()
    for filename in os.listdir(outDir):
        if filename.count("branch") > 0 or filename.count("poke") > 0: # Only want the chunk data for this
            plt.figure(figsize=(35, 10))
            dataset: DataFrame = pandas.read_csv('results/' + filename)
            figure = sns.lineplot(data=dataset, x="y", y="blocksPerSimulation")
            figure.set_xlim([-64, 65])
            figure.set_title(filename.split("_")[0])
            if filename.count("branch") > 0:
                plt.savefig("graphical_results/branch_" + filename.split("_")[0] + ".png")
            else:
                plt.savefig("graphical_results/poke_" + filename.split("_")[0] + ".png")

chunk_data()            
# chunk_graphs()
# techniqueData()
# technique_graphs()

#%% 
# sns.set_theme()
# figure = sns.lineplot(data=list(container["air"].values()))
# figure.set_xlim([-64, 400])
# plt.show

#%%
# sns.set_theme()
# for block in container:
#     figure = sns.lineplot(data=list(container[block].values()))
#     figure.set_title(block)
#     plt.show()


# def chunks():
#     container: Dict[str, Dict[int, int]]
#     for filename in os.listdir(chunkDir):
#         with open('chunk_data/' + filename) as file:
#             data: DataFrame = pandas.read_csv(file)
#             print(data.describe())

# chunks()

# directory = r'.\\results'

# basic = []
# poke = []

# for filename in os.listdir(directory):
#     with open('results\\' + filename) as file:
#         contents = csv.DictReader(file)
#         for line in contents:
#             if filename.count("basic") > 0:
#                 basic.append(line)
#             else:
#                 poke.append(line)

# basicAverages = []
# pokeAverages = []

# for y in range(-59, 64):
#     data = []
#     sumBasic = {"y": y, "blocks": 10420,"lava": 0, "coal": 0, "copper": 0, "iron": 0, "lapis": 0, "redstone": 0, "gold": 0, "emeralds": 0, "diamonds": 0}
#     sumPoke = {"y": y, "blocks": 10234,"lava": 0, "coal": 0, "copper": 0, "iron": 0, "lapis": 0, "redstone": 0, "gold": 0, "emeralds": 0, "diamonds": 0}
#     for x in basic:
#         if int(x['y']) == y:
#             for key in list(x)[2:]:
#                 sumBasic[key] += int(x[key])
#     for key in list(x)[2:]:
#         sumBasic[key] = sumBasic[key] // 26
#     for x in poke:
#         if int(x['y']) == y:
#             for key in list(x)[2:]:
#                 sumPoke[key] += int(x[key])
#     for key in list(x)[2:]:
#         sumPoke[key] = sumPoke[key] // 26
#     basicAverages.append(sumBasic)
#     pokeAverages.append(sumPoke)

# # print(basicAverages)
# # print(pokeAverages)

# ys = []
# avgs = []
# blocks = []
# for ylevel in basicAverages:
#     for block in list(ylevel)[2:]:
#         ys.append(ylevel['y'])
#         avgs.append(ylevel[block])
#         blocks.append(block)

# basicFrame = pandas.DataFrame({'y': ys, 'avgs': avgs, 'block': blocks})

# ys = []
# avgs = []
# blocks = []
# for ylevel in pokeAverages:
#     for block in list(ylevel)[2:]:
#         ys.append(ylevel['y'])
#         avgs.append(ylevel[block])
#         blocks.append(block)

# pokeFrame = pandas.DataFrame({'y': ys, 'avgs': avgs, 'block': blocks})

# # print(pokeFrame)

# blocks = []
# for block in basicFrame['block'].unique():
#     blocks.append(basicFrame.loc[basicFrame['block'] == block])

# sns.set_theme()
# for block in blocks:
#     plt.figure(figsize=(35, 10))
#     figure = sns.barplot(data=block, x="y", y="avgs")
#     figure.set_title('basic-{}'.format(block['block'].unique()[0]))
#     figure.set_xlabel("y level")
#     figure.set_ylabel("Avg blocks found")
#     plt.savefig('basic-{}.png'.format(block['block'].unique()[0]), dpi=400)
#     plt.close()

# blocks = []
# for block in pokeFrame['block'].unique():
#     blocks.append(pokeFrame.loc[pokeFrame['block'] == block])

# for block in blocks:
#     plt.figure(figsize=(35, 10))
#     figure = sns.barplot(data=block, x="y", y="avgs")
#     figure.set_title('poke-{}'.format(block['block'].unique()[0]))
#     figure.set_xlabel("y level")
#     figure.set_ylabel("Avg blocks found")
#     plt.savefig('poke-{}.png'.format(block['block'].unique()[0]), dpi=400)
#     plt.close()

# %%
