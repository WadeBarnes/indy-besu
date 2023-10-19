// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import { StrSlice, toSlice } from "@dk1a/solidity-stringutils/src/StrSlice.sol";
import { FieldRequired, InvalidCredentialDefinitionId, UnsupportedCredentialDefintionType } from "./ClErrors.sol";
import { CredentialDefinition } from "./CredentialDefinitionTypes.sol";
import { CredentialDefinitionRegistryInterface } from "./CredentialDefinitionRegistryInterface.sol";

using { toSlice } for string;

library CredentialDefinitionValidator {
    string private constant DELIMITER = "/";
    string private constant CRED_DEF_ID_MIDDLE_PART = "/anoncreds/v0/CLAIM_DEF/";
    string private constant ANONCREDS_TYPE = "CL";
    
    /**
     * @dev Validates the Credential Definition syntax
     */
    function requireValidId(CredentialDefinition memory self) internal pure {
        string memory credDefId = string.concat(self.issuerId, CRED_DEF_ID_MIDDLE_PART, self.schemaId, DELIMITER, self.tag);

        if (!credDefId.toSlice().eq(self.id.toSlice())) revert InvalidCredentialDefinitionId(self.id);
    }

    /**
     * @dev Validates the Credential Definition type
     */
    function requireValidType(CredentialDefinition memory self) internal pure {
        if (!self.credDefType.toSlice().eq(ANONCREDS_TYPE.toSlice())) {
            revert UnsupportedCredentialDefintionType(self.credDefType);
        }
    }

    /**
     * @dev Validates that the Credential Definition tag is provided
     */
    function requireTag(CredentialDefinition memory self) internal pure {
        if (self.tag.toSlice().isEmpty()) revert FieldRequired("tag"); 
    }

     /**
     * @dev Validates that the Credential Definition value is provided
     */
    function requireValue(CredentialDefinition memory self) internal pure {
        if (self.value.toSlice().isEmpty()) revert FieldRequired("value"); 
    }
}