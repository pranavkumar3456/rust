Difference between the constants and shadowing
is that we are effectively creating new variable when we use let keyword again,
we can change the type of value by reusing the same name.

eg: --> let spaces = " ";
let spaces = spaces.len();

// first spaces variable type was string
// later spaces variable is number type.

so having to comeup with the different name like spaces-num and spaces-str
we can just use the simply spaces.

if we try to mut type for this, we will get an complie-time error.

