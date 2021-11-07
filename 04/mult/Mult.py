#!python3
print('Computes 12 * 6')
print('bin(12) : ', bin(12))
print('bin(6) : ', bin(6))
print('bin(12*6) : ', bin(12*6))

R0 = 12
R1 = 6
R2 = 0

target = 1
R3 = R0

for _ in range(0, 15):
    if (R1 & target) > 0:
        R2 += R3
    R3 += R3
    target += target

print('R2 : ', R2)
print('bin(R2) : ', bin(R2))
