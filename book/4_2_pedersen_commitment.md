# Pedersen Commitment

## Abstract
The `Pedersen Commitment` is the technology which allows us to check the transfer amount is valid without revealing actual amount.

## Details
This technology uses additivity of elliptic curve. Hiding the transfer amount as scalar of elliptic curve point and mixing random value refer as to `blinding factor`. Let's take a look the squence.

1. Setup the parameters
2. Hide the transfer amount
3. Verify the transfer amount

Above sequence used for confidential transfer to keep the transfer amount secret.

## Setup The Parameters
First of all, we'd like to setup the parameters we are going to use with the `Pedersen Commitment`. Selecting generator `G` over prime order elliptic curve group and randomness `a` less than order prime. Calculating `H = aG` and making `H` and `G` public. We can't predict the `a` value from `H` and `G` because of discrete logarithm.

Variable | Explanation | Derivation
:------------ | :------------- | :-------------
p | prime number | -
a | random number | a ∈ Fp
C(x) | elliptic curve function | -
G | generator of elliptic curve point | G ∈ C(Fp)
H | generator made by random a | H = aG

## Hide The Transfer Amount
Let's assume that Alice has `10` balance and send Bob to `3`. We need to check `{Alice balance} - {transfer amount} = {Alice after balance}` without revealing any information about Alice balance. Alice knows `H`, `G` and her balance so she computes following value.

- Alice balance commitment
$$ C(10) = 10H + x_1G $$
- Transfer amount commitment
$$ C(3) = 3H + x_2G $$
- Alice after balance commitment
$$ C(10 - 3) = 7H + x_3G $$

In above equation, `x_1 ~ x_3` are called `blinding factor` and each blinding factor need to be set to hold `{Alice balance} - {transfer amount} = {Alice after balance}` equation. Let's say `x_1 = 330`, `x_2 = 30` and `x_3 = 300`.

## Verify The Transfer Amount
In previous section, Alice computes the following commitment.

$$ C(10, 330) = 10H + 330G $$
$$ C(3, 30) = 3H + 30G $$
$$ C(10 - 3, 300) = 7H + 300G $$

Let's check if Alice transfer amount is valid. The elliptic curve arithmetic supports additive so we can check `{Alice balance} - {transfer amount} = {Alice after balance}` as following.

$$ C(10, 330) - C(3, 30) - C(10 - 3, 300) = C(10 - 3 - 7, 330 - 30 - 300) = 0H + 0G = 0 $$

If above equation holds up, we can know the transfer amount is valid. Through this process, `balance` and `transfer amount` are encrypted by elliptic curve so no one can predict actual value from public information. This is how we conceal the transfer transaction and verify the validity.
