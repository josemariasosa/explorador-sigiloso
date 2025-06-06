// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

interface IGnosisSafeProxyFactory {
    function createProxyWithNonce(
        address singleton,
        bytes memory initializer,
        uint256 saltNonce
    ) external returns (address proxy);
}

contract GnosisSafeTxBuilder {
    /// @notice Returns the raw data to be signed and sent by a Safe Module
    /// @param factory The Gnosis Proxy Factory address
    /// @param singleton The address of the Gnosis Safe Master Copy (v1.4.1)
    /// @param owners The list of initial owners for the Safe
    /// @param threshold The number of confirmations needed
    /// @param saltNonce A user-supplied nonce to make the deployment deterministic
    function getCreateSafeTx(
        address factory,
        address singleton,
        address[] calldata owners,
        uint256 threshold,
        uint256 saltNonce
    ) external pure returns (address to, uint256 value, bytes memory data) {
        // Build initializer
        bytes memory initializer = abi.encodeWithSignature(
            "setup(address[],uint256,address,bytes,address,address,uint256,address,address)",
            owners,
            threshold,
            address(0), // to
            bytes(""),  // data
            address(0), // fallbackHandler
            address(0), // paymentToken
            0,          // payment
            address(0), // paymentReceiver
            address(0)  // unused
        );

        // Build transaction data
        bytes memory txData = abi.encodeWithSelector(
            IGnosisSafeProxyFactory.createProxyWithNonce.selector,
            singleton,
            initializer,
            saltNonce
        );

        return (factory, 0, txData);
    }
}
