import os
import csv
import seaborn as sns
import pandas
import matplotlib.pyplot as plt

directory = r'.\\results'

basic = []
poke = []

for filename in os.listdir(directory):
    with open('results\\' + filename) as file:
        contents = csv.DictReader(file)
        for line in contents:
            if filename.count("basic") > 0:
                basic.append(line)
            else:
                poke.append(line)

basicAverages = []
pokeAverages = []

for y in range(-59, 64):
    data = []
    sumBasic = {"y": y, "blocks": 10420,"lava": 0, "coal": 0, "copper": 0, "iron": 0, "lapis": 0, "redstone": 0, "gold": 0, "emeralds": 0, "diamonds": 0}
    sumPoke = {"y": y, "blocks": 10234,"lava": 0, "coal": 0, "copper": 0, "iron": 0, "lapis": 0, "redstone": 0, "gold": 0, "emeralds": 0, "diamonds": 0}
    for x in basic:
        if int(x['y']) == y:
            for key in list(x)[2:]:
                sumBasic[key] += int(x[key])
    for key in list(x)[2:]:
        sumBasic[key] = sumBasic[key] // 26
    for x in poke:
        if int(x['y']) == y:
            for key in list(x)[2:]:
                sumPoke[key] += int(x[key])
    for key in list(x)[2:]:
        sumPoke[key] = sumPoke[key] // 26
    basicAverages.append(sumBasic)
    pokeAverages.append(sumPoke)

# print(basicAverages)
# print(pokeAverages)

ys = []
avgs = []
blocks = []
for ylevel in basicAverages:
    for block in list(ylevel)[2:]:
        ys.append(ylevel['y'])
        avgs.append(ylevel[block])
        blocks.append(block)

basicFrame = pandas.DataFrame({'y': ys, 'avgs': avgs, 'block': blocks})

ys = []
avgs = []
blocks = []
for ylevel in pokeAverages:
    for block in list(ylevel)[2:]:
        ys.append(ylevel['y'])
        avgs.append(ylevel[block])
        blocks.append(block)

pokeFrame = pandas.DataFrame({'y': ys, 'avgs': avgs, 'block': blocks})

# print(pokeFrame)

blocks = []
for block in basicFrame['block'].unique():
    blocks.append(basicFrame.loc[basicFrame['block'] == block])

sns.set_theme()
for block in blocks:
    plt.figure(figsize=(35, 10))
    figure = sns.barplot(data=block, x="y", y="avgs")
    figure.set_title('basic-{}'.format(block['block'].unique()[0]))
    figure.set_xlabel("y level")
    figure.set_ylabel("Avg blocks found")
    plt.savefig('basic-{}.png'.format(block['block'].unique()[0]), dpi=400)
    plt.close()

blocks = []
for block in pokeFrame['block'].unique():
    blocks.append(pokeFrame.loc[pokeFrame['block'] == block])

for block in blocks:
    plt.figure(figsize=(35, 10))
    figure = sns.barplot(data=block, x="y", y="avgs")
    figure.set_title('poke-{}'.format(block['block'].unique()[0]))
    figure.set_xlabel("y level")
    figure.set_ylabel("Avg blocks found")
    plt.savefig('poke-{}.png'.format(block['block'].unique()[0]), dpi=400)
    plt.close()