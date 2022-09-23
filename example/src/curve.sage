# same prime number with jubjub finite field
p = 6554484396890773809930967563523245729705921265872317281365359162392183254199
F = GF(p)

# bls12-381 curve params
a = 0
b = 4
E = EllipticCurve(F, [a, b])

class Bls12_381:
    def __init__(self):
        self.F = F
        self.E = E
    def generator(self):
        return self.E([0xc4c71e5a410296003b3fe93f505a6f272e38f4ebd4070f37c24d812779a3316, 0xaa87a50921b80ecd3033593ae386e9201ca03640d236cbfd2047ef3463de4af])
    def random_point(self):
        return self.E.random_point()

bls12_381 = Bls12_381()
print(bls12_381.random_point())
