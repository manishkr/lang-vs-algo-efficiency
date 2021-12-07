import random
for i in range(1, 8):
    data = [str(random.randint(1, 10**i)) for _ in range(10**i)]
    with open(f"inputs/input_{i}.txt", 'w') as writer:
        writer.write('\n'.join(data))
