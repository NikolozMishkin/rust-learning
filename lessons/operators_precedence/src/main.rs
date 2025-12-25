/*
Operator                             Asspciativity
* / %                                left to right
+ -                                  left to right
<< >>                                left to right
&                                    left to right
^                                    left to right
|                                    left to right
== != < > <= >=                      requares parentheses
&&                                   left to right
||                                   left to right
.. ..=                               requares parentheses
= += -= *= /= %= <<= >>= &= ^= |=    right to left
*/

fn main() {
    let result = 5 + 2 * 3; //5 + (2 * 3)
    let expression = 16 / 4 * 2; // (16 / 4) * 2

    let x = (5 + 2) * 3; // parentheses change the precedence
    let a = 3;
    let b = 4;
    let c = 5;
    let result = a + b - c == 2 || a == 3; // 1. a + b = 7 // 2. 7 - c = 2 // 3. 2 == 2 -> true // 4. a == 3 -> true // true || true -> true
}
