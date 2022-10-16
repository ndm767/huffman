# Book Example Output

The is the output when the program is run with book_probs.csv.  

## Encode Radix 2

```rs
$ cargo run book_probs.csv encode bookprob_plaintext.txt 
Alphabet: [('2', 0.05), ('1', 0.06), ('b', 0.1), ('c', 0.19), ('d', 0.25), ('a', 0.35)]
Code: [('a', "11"), ('d', "10"), ('1', "0111"), ('2', "0110"), ('b', "010"), ('c', "00")]
Original message:
abcddcba
Encoded message:
110100010100001011
Alphabet: [('2', 0.05), ('1', 0.06), ('b', 0.1), ('c', 0.19), ('d', 0.25), ('a', 0.35)]
Code: [('a', "11"), ('d', "10"), ('1', "0111"), ('2', "0110"), ('b', "010"), ('c', "00")]
```

## Decode Radix 2

```rs
$ cargo run book_probs.csv decode bookprob_encoded.txt
Alphabet: [('2', 0.05), ('1', 0.06), ('b', 0.1), ('c', 0.19), ('d', 0.25), ('a', 0.35)]
Code: [('a', "11"), ('d', "10"), ('1', "0111"), ('2', "0110"), ('b', "010"), ('c', "00")]
Original message:
110100010100001011
Decoded message:
abcddcba
Alphabet: [('2', 0.05), ('1', 0.06), ('b', 0.1), ('c', 0.19), ('d', 0.25), ('a', 0.35)]
Code: [('c', "00"), ('d', "10"), ('a', "11"), ('b', "010"), ('2', "0110"), ('1', "0111")]
```

## Encode Radix 3

```rs
$ cargo run book_probs.csv encode "abcddcba" 3                   
Alphabet: [('2', 0.05), ('1', 0.06), ('b', 0.1), ('c', 0.19), ('d', 0.25), ('a', 0.35)]
Code: [('a', "0"), ('c', "10"), ('d', "12"), ('2', "110"), ('1', "111"), ('b', "112")]
Original message:
abcddcba
Encoded message:
0112101212101120
Alphabet: [('2', 0.05), ('1', 0.06), ('b', 0.1), ('c', 0.19), ('d', 0.25), ('a', 0.35)]
Code: [('a', "0"), ('c', "10"), ('d', "12"), ('2', "110"), ('1', "111"), ('b', "112")]
```

## Decode Radix 3

```rs
$ cargo run book_probs.csv decode "0112101212101120" 3                
Alphabet: [('2', 0.05), ('1', 0.06), ('b', 0.1), ('c', 0.19), ('d', 0.25), ('a', 0.35)]
Code: [('a', "0"), ('c', "10"), ('d', "12"), ('2', "110"), ('1', "111"), ('b', "112")]
Original message:
0112101212101120
Decoded message:
abcddcba
Alphabet: [('2', 0.05), ('1', 0.06), ('b', 0.1), ('c', 0.19), ('d', 0.25), ('a', 0.35)]
Code: [('a', "0"), ('c', "10"), ('d', "12"), ('2', "110"), ('1', "111"), ('b', "112")]
```
