import csv

i = 0
names = []

with open('data/pokemon.txt', 'r') as file:
    names = file.read().splitlines()

with open('data/list.csv', 'w', newline='') as file:
    writer = csv.writer(file)
    
    for name in names:
        filename = name.lower().replace(' ', "-").replace('_', '-').replace('.', "").replace('\'', "").replace(":", "")
        writer.writerow([name, filename])

    
