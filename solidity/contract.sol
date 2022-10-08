pragma solidity ^0.8.0;

contract Addition{

	int public x;
    bool public large;
    function add() public {
        int a = 5;
        int b = 6;
    	x = a + b;

        if (x < 5) {
            x = 5;
        } else {
            x = 7;
        }
   	}
}