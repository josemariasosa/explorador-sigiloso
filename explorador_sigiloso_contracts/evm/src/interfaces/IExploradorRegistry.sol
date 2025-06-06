// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

interface IExploradorRegistry {
    struct Node {
        address operator;
        string chain;
        uint256 registeredAt;
        uint256 lastClaimed;
    }

    function registerNode(string calldata chain) external;
    function claimReward() external;
    function getNode(address operator) external view returns (Node memory);
}
