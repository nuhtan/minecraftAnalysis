import os
from typing import Dict
from pandas.core.frame import DataFrame
import pandas

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
    air.to_csv("results/chunks_air_full_range.csv")
    for subset in container:
        blockData = container[subset]
        blockData[blockData['y'] <= 62].to_csv("results/" + subset + "_chunks.csv", index=False)
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
