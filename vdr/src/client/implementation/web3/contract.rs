use crate::{
    client::{implementation::web3::client::Web3Client, Contract, ContractOutput, ContractSpec},
    error::{VdrError, VdrResult},
};

use std::str::FromStr;
use web3::{
    contract::Contract as Web3ContractImpl,
    ethabi::{Address, Function, Token},
    transports::Http,
};

pub struct Web3Contract {
    address: String,
    contract: Web3ContractImpl<Http>,
}

impl Web3Contract {
    pub fn new(
        web3_client: &Web3Client,
        address: &str,
        contract_spec: &ContractSpec,
    ) -> VdrResult<Web3Contract> {
        let abi = serde_json::to_vec(&contract_spec.abi).map_err(|err| {
            VdrError::CommonInvalidData(format!(
                "Unable to parse contract ABI from specification. Err: {:?}",
                err.to_string()
            ))
        })?;
        let parsed_address = Address::from_str(address).map_err(|err| {
            VdrError::CommonInvalidData(format!(
                "Unable to parse contract address. Err: {:?}",
                err.to_string()
            ))
        })?;
        let contract =
            Web3ContractImpl::from_json(web3_client.eth(), parsed_address, abi.as_slice())?;
        Ok(Web3Contract {
            contract,
            address: address.to_string(),
        })
    }

    fn function(&self, name: &str) -> VdrResult<&Function> {
        self.contract.abi().function(name).map_err(VdrError::from)
    }
}

impl Contract for Web3Contract {
    fn address(&self) -> String {
        self.address.to_string()
    }

    fn encode_input(&self, method: &str, params: &[Token]) -> VdrResult<Vec<u8>> {
        self.function(method)?
            .encode_input(params)
            .map_err(VdrError::from)
    }

    fn decode_output(&self, method: &str, output: &[u8]) -> VdrResult<ContractOutput> {
        self.function(method)?
            .decode_output(output)
            .map_err(VdrError::from)
            .map(ContractOutput::from)
    }
}
