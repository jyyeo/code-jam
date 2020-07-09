import numpy as np 
import math

T = int(input())

for i in range(T):
	N, L = [int(s) for s in input().split(" ")]
	
	ciphertext = [int(s) for s in input().split(" ")]

	deciphered = []
	second = math.gcd(ciphertext[0], ciphertext[1])
	first = ciphertext[0] / second
	deciphered.append(int(first))
	deciphered.append(int(second))
	for j in range(1,L):
		x = ciphertext[j] / deciphered[j]
		deciphered.append(int(x))

	unique_deciphered = []
	for j in deciphered:
		if j not in unique_deciphered:
			unique_deciphered.append(j)

	sorted_deciphered = unique_deciphered
	sorted_deciphered.sort()

	text = ""
	for j in range(L+1):
		text += chr(sorted_deciphered.index(deciphered[j]) + ord('A'))
	
	print("Case #{}: {}".format(i+1, text))
 
	
