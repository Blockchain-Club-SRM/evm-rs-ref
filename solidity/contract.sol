pragma solidity ^0.8.0;

contract TestingStuff {

	uint public x;

    function add() public {
        uint sum = 0;
        for (uint i = 0; i < 10; i++) {
            sum += i;
        }
        x = sum;
    }
}