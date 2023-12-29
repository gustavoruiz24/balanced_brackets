# Balanced Brackets
I made 3 algorithms, 2 of them solve the balanced brackets problem and the last one take the position of the bracket "partner".

I used 6 exemples to test them:
 - Balanced: "(()()()())", "(((())))", "(()((())()))".
 - Not balanced: "((((((())", "()))", "(()()(()".

## Concept applyed to all
In these algorithms there is a balance that must have the value equals 0. If it is different from 0, there is too much "(" or ")".
From this concept "(" makes the balance more positive and ")" makes the balance more negative. If they have the same quantity it will end neutral.

## The first one
In the "is_balanced_1" the algorithm will verify if the characther equals "(", anything different from this, ")" is the target, makes the balance more negative.
It will return wrong results is receive a expression with values besides "(" and ")".

Here are some examples:
- "(()((())()))" => true
- "((((((())" => false

## The second one
In the "is_balanced_2" the algorithm will verify if the character equals "(" or equals ")" and add the "weight".
So you can pass any expression to this and it will work.

Here are some examples:
- "5*((7+8)-7*(((2+2)+3)/(4+4)))" => true
- "1+(2*(3+(4/(5-(6*(2+2))" => false

## The third one
In "bracket_partner" you don't verify if the brackets are balanced, you pass an index and it return the index of the partner or None.
Don't metter if it is balanced or not, if the bracket has a partner it will return the partner.

Here are some examples:
- The partner of the index 10 in "(()((())()))" is 3
- The partner of the index 5 in "((((((())" is 8
- The partner of the index 2 in "()))" is None
