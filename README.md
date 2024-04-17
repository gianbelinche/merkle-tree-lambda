# Merkle Tree

## Description

Merkle Tree implemented in Rust for Hacking Learning Path at LambdaClass

A Merkle tree, is a tree in which each leaf node is labeled with the cryptographic hash of a data block, and each non-leaf node is labeled with the cryptographic hash of its child nodes' labels.

This Merkle Tree implements the following functionality:
- Creating a new Merkle Tree from a vector of strings
- Calculating the root of the Merkle Tree
- Adding new elements to the Merkle Tree
- Removing an element from the Merkle Tree
- Generating a proof needed to prove if a given value is in the Merkle Tree
- Verifying that a given proof is correct for the given tree

## Usage

Run the following command to execute the tests:

`make test`

For formatting and lint run:

`make lint`

For an example run:

`make example`


## References

https://brilliant.org/wiki/merkle-tree/

https://www.youtube.com/watch?v=n6nEPaE7KZ8



