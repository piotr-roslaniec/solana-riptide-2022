pragma circom 2.0.0;

/*This circuit template checks that c is the multiplication of a and b.*/  

template Multiplier2() {
    signal input a;
    signal input b;
    signal output c;
    c <== a*b;
 }

 component main = Multiplier2();