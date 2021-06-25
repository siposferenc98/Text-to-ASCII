import sys
import math


betuk = []
f = open("alphabet.txt")
l = 4
h = 5
print("Írd be a szövegedet")
t = input()
szamlalo = 0
for i in range(h):
    row = f.readline()
    j = int(0)
    k = int(l)
    betuk.append([])
    while j < len(row):
        substring = row[j:k]
        betuk[i].append(substring)
        j+=l
        k+=l

abc = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',' ','?']
bemenet = t.upper()
indexek = []
i = 0
j = 0
nincs = 0
while i < len(bemenet):
    while j < len(abc):
        nincs+=1
        if abc[j] == bemenet[i]:
            indexek.append(j)
            nincs = 0
        elif nincs == len(abc):
            indexek.append(27)
        j+=1
    j = 0
    nincs = 0
    i+=1

i = 0
j = 0
answer = []
while i < h:
    answer.append([])
    while j < len(indexek):
        answer[i].append(betuk[i][indexek[j]])
        j+=1
    j = 0
    i+=1

i = 0
j = 0


while i < h:
    ki = "".join(answer[i])
    print(ki)
    ki = ""
    i+=1









