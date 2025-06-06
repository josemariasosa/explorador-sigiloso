// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "./interfaces/IExploradorRegistry.sol";

contract ExploradorRegistry is IExploradorRegistry {
    mapping(address => Node) public nodes;
    uint256 public rewardRate = 0.001 ether; // example flat reward

    function registerNode(string calldata chain) external override {
        Node storage n = nodes[msg.sender];
        require(n.registeredAt == 0, "Already registered");
        nodes[msg.sender] = Node({
            operator: msg.sender,
            chain: chain,
            registeredAt: block.timestamp,
            lastClaimed: block.timestamp
        });
    }

    function claimReward() external override {
        Node storage n = nodes[msg.sender];
        require(n.registeredAt > 0, "Not registered");

        uint256 timeElapsed = block.timestamp - n.lastClaimed;
        uint256 reward = (timeElapsed / 1 days) * rewardRate;

        require(reward > 0, "Nothing to claim");
        n.lastClaimed = block.timestamp;
        payable(msg.sender).transfer(reward);
    }

    function getNode(address operator) external view override returns (Node memory) {
        return nodes[operator];
    }

    receive() external payable {} // accepts funding
}
