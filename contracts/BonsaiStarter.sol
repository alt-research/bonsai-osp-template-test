// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.17;

import {IBonsaiRelay} from "bonsai/IBonsaiRelay.sol";
import {BonsaiCallbackReceiver} from "bonsai/BonsaiCallbackReceiver.sol";

/// @title A starter application using Bonsai through the on-chain relay.
/// @dev This contract demonstrates one pattern for offloading the computation of an expensive
//       or difficult to implement function to a RISC Zero guest running on Bonsai.
contract BonsaiStarter is BonsaiCallbackReceiver {
    mapping(bytes32 => bytes32) public stateCache;
    
    /// @notice the inst root for execute
    bytes32 public instRoot;

    /// @notice the func root for execute
    bytes32 public funcRoot;

    /// @notice Image ID of the only zkVM binary to accept callbacks from.
    bytes32 public immutable imageID;

    /// @notice Gas limit set on the callback from Bonsai.
    /// @dev Should be set to the maximum amount of gas your callback might reasonably consume.
    uint64 private constant BONSAI_CALLBACK_GAS_LIMIT = 100000;

    /// @notice Initialize the contract, binding it to a specified Bonsai relay and RISC Zero guest image.
    constructor(IBonsaiRelay bonsaiRelay, bytes32 _imageID, bytes32 _instRoot, bytes32 _funcRoot) BonsaiCallbackReceiver(bonsaiRelay) {
        imageID = _imageID;
        instRoot = _instRoot;
        funcRoot = _funcRoot;
    }

    event ExecuteOneStepCallback(bytes32 instRoot, bytes32 funcRoot, bytes32 indexed preState, bytes32 postState);

    /// @notice Returns the post state after execute one step based on per state.
    function getPostState(bytes32 preState) external view returns (bytes32) {
        bytes32 result = stateCache[preState];
        require(result != 0, "value not available in cache");
        return result;
    }

    /// @notice Callback function logic for processing verified journals from Bonsai.
    function storeResult(bytes32 preState, bytes32 postState) external onlyBonsaiCallback(imageID) {
        emit ExecuteOneStepCallback(instRoot, funcRoot, preState, postState);
        stateCache[preState] = postState;
    }

    /// @notice Sends a request to Bonsai to have have the executeOneStep return
    function executeOneStep(bytes calldata proof) external {
        bonsaiRelay.requestCallback(
            imageID, abi.encode(instRoot, funcRoot, proof), address(this), this.storeResult.selector, BONSAI_CALLBACK_GAS_LIMIT
        );
    }
}
