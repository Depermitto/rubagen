extern crate lazy_static;
extern crate bimap;
use bimap::BiMap;

use lazy_static::lazy_static;

// 1 - 1px line
// 2 - 2px line
// Code 39 has 9 lines + 1 white after = 10
lazy_static! (
    pub static ref CODES39: BiMap<char, [u32; 10]> = {
        BiMap::from_iter([
            ('0', [1,1,1,3,3,1,3,1,1, 1]),
            ('1', [3,1,1,3,1,1,1,1,3, 1]),
            ('2', [1,1,3,3,1,1,1,1,3, 1]),
            ('3', [3,1,3,3,1,1,1,1,1, 1]),
            ('4', [1,1,1,3,3,1,1,1,3, 1]),
            ('5', [3,1,1,3,3,1,1,1,1, 1]),
            ('6', [1,1,3,3,3,1,1,1,1, 1]),
            ('7', [1,1,1,3,1,1,3,1,3, 1]),
            ('8', [3,1,1,3,1,1,3,1,1, 1]),
            ('9', [1,1,3,3,1,1,3,1,1, 1]),
            ('A', [3,1,1,1,1,3,1,1,3, 1]),
            ('B', [1,1,3,1,1,3,1,1,3, 1]),
            ('C', [3,1,3,1,1,3,1,1,1, 1]),
            ('D', [1,1,1,1,3,3,1,1,3, 1]),
            ('E', [3,1,1,1,3,3,1,1,1, 1]),
            ('F', [1,1,3,1,3,3,1,1,1, 1]),
            ('G', [1,1,1,1,1,3,3,1,3, 1]),
            ('H', [3,1,1,1,1,3,3,1,1, 1]),
            ('I', [1,1,3,1,1,3,3,1,1, 1]),
            ('J', [1,1,1,1,3,3,3,1,1, 1]),
            ('K', [3,1,1,1,1,1,1,3,3, 1]),
            ('L', [1,1,3,1,1,1,1,3,3, 1]),
            ('M', [3,1,3,1,1,1,1,3,1, 1]),
            ('N', [1,1,1,1,3,1,1,3,3, 1]),
            ('O', [3,1,1,1,3,1,1,3,1, 1]),
            ('P', [1,1,3,1,3,1,1,3,1, 1]),
            ('Q', [1,1,1,1,1,1,3,3,3, 1]),
            ('R', [3,1,1,1,1,1,3,3,1, 1]),
            ('S', [1,1,3,1,1,1,3,3,1, 1]),
            ('T', [1,1,1,1,3,1,3,3,1, 1]),
            ('U', [3,3,1,1,1,1,1,1,3, 1]),
            ('V', [1,3,3,1,1,1,1,1,3, 1]),
            ('W', [3,3,3,1,1,1,1,1,1, 1]),
            ('X', [1,3,1,1,3,1,1,1,3, 1]),
            ('Y', [3,3,1,1,3,1,1,1,1, 1]),
            ('Z', [1,3,3,1,3,1,1,1,1, 1]),
            (' ', [1,3,3,1,1,1,3,1,1, 1]),
            ('-', [1,3,1,1,1,1,3,1,3, 1]),
            ('$', [1,3,1,3,1,3,1,1,1, 1]),
            ('+', [1,3,1,1,1,3,1,3,1, 1]),
            ('.', [3,3,1,1,1,1,3,1,1, 1]),
            ('/', [1,3,1,3,1,1,1,3,1, 1]),
            ('%', [1,1,1,3,1,3,1,3,1, 1]),
            ('*', [1,3,1,1,3,1,3,1,1, 1]),
        ])
    };
);