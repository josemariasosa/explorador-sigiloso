// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import "../src/ExploradorRegistry.sol";

contract ExploradorRegistryTest is Test {
    ExploradorRegistry registry;

    function setUp() public {
        registry = new ExploradorRegistry();
        vm.deal(address(this), 10 ether);
        (bool ok,) = address(registry).call{value: 5 ether}("");
        require(ok, "Funding failed");
    }

    function testRegisterAndClaim() public {
        registry.registerNode("bitcoin");
        skip(2 days);
        registry.claimReward();
        assertGt(address(this).balance, 5 ether, "Should receive some reward");
    }
}
