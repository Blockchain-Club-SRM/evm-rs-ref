pragma solidity ^0.8.0;

contract Addition{

	int public x;
    
    function add() public {
        int a = 5;
        int b = 6;
    	x = a + b;
   	}
}