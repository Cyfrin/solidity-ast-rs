// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.34;

contract Counter {
    uint256 public number;

    function setNumber(uint256 newNumber) public {
        number = newNumber;
    }

    function leadingZeros(uint256 bitmap) public pure returns (uint256 leadingZeroCount) {
        assembly {
            leadingZeroCount := clz(bitmap)
        }
    }
}
