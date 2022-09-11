# same prime number with circom finite field
p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
F = GF(p)
g = F(2)

def private_key():
    return F.random_element()

def public_key(private_key):
    return g^private_key

def shared_secret(public_key, private_key):
    return public_key^private_key

def encrypt(message, shared_secret):
    return message * shared_secret

def decrypt(encrypted_message, shared_secret):
    return encrypted_message / shared_secret

def elgamal_scheme():
    alice_sk = private_key()
    alice_pk = public_key(alice_sk)

    bob_sk = private_key()
    bob_pk = public_key(bob_sk)

    alice_shared_secret = shared_secret(bob_pk, alice_sk)
    alice_secret_message = F.random_element()
    alice_encrypted_message = encrypt(alice_secret_message, alice_shared_secret)

    bob_shared_secret = shared_secret(alice_pk, bob_sk)
    bob_decrypted_message = decrypt(alice_encrypted_message, bob_shared_secret)

    assert(alice_shared_secret == bob_shared_secret)
    assert(alice_secret_message == bob_decrypted_message)

elgamal_scheme()
