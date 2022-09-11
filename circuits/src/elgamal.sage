class ElGamal:
    def __init__(self):
        # same prime number with circom finite field
        p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
        F = GF(p)
        self.F = F
        self.g = F.gen()

    def randomness(self):
        return self.F.random_element()

    def private_key(self):
        return self.randomness()

    def public_key(self, private_key):
        return self.g^private_key

    @staticmethod
    def shared_secret(public_key, private_key):
        return public_key^private_key

    @staticmethod
    def sign_message(message, shared_secret):
        return message * shared_secret

    @staticmethod
    def decrypt(encrypted_message, shared_secret):
        return encrypted_message / shared_secret

    def encrypt(self, message, randomness, public_key):
        return (self.g^randomness, (self.g^message) * (public_key^randomness))

def additive_homomorphic():
    elgamal = ElGamal()
    alice_sk = elgamal.private_key()
    alice_pk = elgamal.public_key(alice_sk)

    value = elgamal.randomness()
    value_prime = elgamal.randomness()
    random = elgamal.randomness()
    random_prime = elgamal.randomness()

    (encrypted_g, encrypted_message) = elgamal.encrypt(value, random, alice_pk)
    (encrypted_g_prime, encrypted_message_prime) = elgamal.encrypt(value_prime, random_prime, alice_pk)
    encrypted_sum = (encrypted_g * encrypted_g_prime, encrypted_message * encrypted_message_prime)

    value_sum = value + value_prime
    random_sum = random + random_prime
    sum_encrypted = elgamal.encrypt(value_sum, random_sum, alice_pk)

    assert(encrypted_sum == sum_encrypted)

additive_homomorphic()

def dh_key_exchange():
    elgamal = ElGamal()
    alice_sk = elgamal.private_key()
    alice_pk = elgamal.public_key(alice_sk)

    bob_sk = elgamal.private_key()
    bob_pk = elgamal.public_key(bob_sk)

    alice_shared_secret = elgamal.shared_secret(bob_pk, alice_sk)
    alice_secret_message = elgamal.randomness()
    alice_encrypted_message = elgamal.sign_message(alice_secret_message, alice_shared_secret)

    bob_shared_secret = elgamal.shared_secret(alice_pk, bob_sk)
    bob_decrypted_message = elgamal.decrypt(alice_encrypted_message, bob_shared_secret)

    assert(alice_shared_secret == bob_shared_secret)
    assert(alice_secret_message == bob_decrypted_message)

dh_key_exchange()
