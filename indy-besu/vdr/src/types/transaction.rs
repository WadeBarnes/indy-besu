use ethereum::{
    LegacyTransaction, LegacyTransactionMessage, TransactionAction,
    TransactionSignature as EthTransactionSignature,
};
use ethereum_types::{H160, H256, U256};
use log::{trace, warn};
use serde_derive::{Deserialize, Serialize};
use std::{str::FromStr, sync::RwLock};

use crate::{
    client::{GAS_LIMIT, GAS_PRICE},
    error::{VdrError, VdrResult},
    types::{Address, ContractOutput, ContractParam},
    LedgerClient,
};

/// Type of transaction: write/read
/// depending on the transaction type different client methods will be executed to submit transaction
#[derive(Clone, Debug, Default, PartialEq)]
pub enum TransactionType {
    #[default]
    Read,
    Write,
}

/// Transaction object
#[derive(Debug, Default)]
pub struct Transaction {
    /// type of transaction: write/read
    /// depending on the transaction type different client methods will be executed to submit transaction
    pub type_: TransactionType,
    /// transaction sender account address
    pub from: Option<Address>,
    /// transaction recipient address
    pub to: Address,
    /// nonce - count of transaction sent by account
    pub nonce: Option<Vec<u64>>,
    /// chain id of the ledger
    pub chain_id: u64,
    /// transaction payload
    pub data: Vec<u8>,
    /// transaction signature
    pub signature: RwLock<Option<TransactionSignature>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignatureData {
    /// recovery ID using for public key recovery
    pub recovery_id: u64,
    /// ECDSA signature
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionSignature {
    pub v: u64,
    pub r: Vec<u8>,
    pub s: Vec<u8>,
}

impl Transaction {
    pub fn new(
        type_: TransactionType,
        from: Option<Address>,
        to: Address,
        chain_id: u64,
        data: Vec<u8>,
        nonce: Option<Vec<u64>>,
        signature: Option<TransactionSignature>,
    ) -> Transaction {
        Transaction {
            type_,
            from,
            to,
            chain_id,
            data,
            nonce,
            signature: RwLock::new(signature),
        }
    }

    pub fn get_signing_bytes(&self) -> VdrResult<Vec<u8>> {
        let eth_transaction: LegacyTransactionMessage = self.try_into()?;
        let hash = eth_transaction.hash();
        Ok(hash.as_bytes().to_vec())
    }

    pub fn set_signature(&self, signature_data: SignatureData) {
        let v = signature_data.recovery_id + 35 + self.chain_id * 2;
        let transaction_signature = TransactionSignature {
            v,
            r: signature_data.signature[..32].to_vec(),
            s: signature_data.signature[32..].to_vec(),
        };
        let mut signature = self.signature.write().unwrap();
        *signature = Some(transaction_signature)
    }

    fn get_to(&self) -> VdrResult<H160> {
        H160::from_str(&self.to).map_err(|_| VdrError::ClientInvalidTransaction {
            msg: format!("Invalid transaction target address {:?}", self.to),
        })
    }

    fn get_nonce(&self) -> VdrResult<U256> {
        let nonce: [u64; 4] = self
            .nonce
            .as_ref()
            .ok_or_else(|| VdrError::ClientInvalidTransaction {
                msg: "Transaction `nonce` is not set".to_string(),
            })?
            .clone()
            .try_into()
            .map_err(|_| VdrError::CommonInvalidData {
                msg: "Invalid nonce provided".to_string(),
            })?;
        Ok(U256(nonce))
    }

    fn get_transaction_signature(&self) -> VdrResult<EthTransactionSignature> {
        let signature = self.signature.read().unwrap();
        let signature = signature
            .as_ref()
            .ok_or_else(|| VdrError::ClientInvalidTransaction {
                msg: "Missing signature".to_string(),
            })?
            .clone();

        let signature = EthTransactionSignature::new(
            signature.v,
            H256::from_slice(&signature.r),
            H256::from_slice(&signature.s),
        )
        .ok_or_else(|| VdrError::ClientInvalidTransaction {
            msg: "Transaction `nonce` is not set".to_string(),
        })?;
        Ok(signature)
    }
}

impl TryInto<LegacyTransactionMessage> for &Transaction {
    type Error = VdrError;

    fn try_into(self) -> Result<LegacyTransactionMessage, Self::Error> {
        Ok(LegacyTransactionMessage {
            nonce: self.get_nonce()?,
            gas_price: *GAS_PRICE,
            gas_limit: *GAS_LIMIT,
            action: TransactionAction::Call(self.get_to()?),
            value: Default::default(),
            input: self.data.clone(),
            chain_id: Some(self.chain_id),
        })
    }
}

impl TryInto<LegacyTransaction> for &Transaction {
    type Error = VdrError;

    fn try_into(self) -> Result<LegacyTransaction, Self::Error> {
        Ok(LegacyTransaction {
            nonce: self.get_nonce()?,
            gas_price: *GAS_PRICE,
            gas_limit: *GAS_LIMIT,
            action: TransactionAction::Call(self.get_to()?),
            value: Default::default(),
            input: self.data.clone(),
            signature: self.get_transaction_signature()?,
        })
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        let self_signature = self.signature.read().unwrap();
        let other_signature = other.signature.read().unwrap();
        self.type_ == other.type_
            && self.from == other.from
            && self.to == other.to
            && self.nonce == other.nonce
            && self.chain_id == other.chain_id
            && self.data == other.data
            && *self_signature == *other_signature
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TransactionBuilder {
    contract: String,
    method: String,
    from: Option<Address>,
    nonce: Option<[u64; 4]>,
    params: Vec<ContractParam>,
    type_: TransactionType,
}

impl TransactionBuilder {
    pub fn new() -> TransactionBuilder {
        TransactionBuilder::default()
    }

    pub fn set_contract(mut self, contract: &str) -> TransactionBuilder {
        self.contract = contract.to_string();

        trace!(
            "Set contract: {} to TransactionBuilder: {:?}",
            contract,
            self
        );

        self
    }

    pub fn set_method(mut self, method: &str) -> TransactionBuilder {
        self.method = method.to_string();

        trace!("Set method: {} to TransactionBuilder: {:?}", method, self);

        self
    }

    pub fn add_param(mut self, param: ContractParam) -> TransactionBuilder {
        self.params.push(param.clone());

        trace!(
            "Added ContractParam: {:?} to TransactionBuilder: {:?}",
            param,
            self
        );

        self
    }

    pub fn set_type(mut self, type_: TransactionType) -> TransactionBuilder {
        self.type_ = type_.clone();

        trace!(
            "Set TransactionType: {:?} to TransactionBuilder: {:?}",
            type_,
            self
        );

        self
    }

    pub fn set_from(mut self, from: &Address) -> TransactionBuilder {
        self.from = Some(from.clone());

        trace!("Set from: {:?} to TransactionBuilder: {:?}", from, self);

        self
    }

    pub async fn build(self, client: &LedgerClient) -> VdrResult<Transaction> {
        let contract = client.contract(&self.contract)?;
        let data = contract.encode_input(&self.method, &self.params)?;
        let nonce = match self.type_ {
            TransactionType::Write => {
                let from =
                    self.from
                        .as_ref()
                        .ok_or_else(|| VdrError::ClientInvalidTransaction {
                            msg: "Transaction `sender` is not set".to_string(),
                        })?;

                let nonce = client.get_transaction_count(from).await?;
                Some(nonce.to_vec())
            }
            TransactionType::Read => None,
        };

        let transaction = Transaction {
            type_: self.type_,
            from: self.from,
            to: contract.address().clone(),
            chain_id: client.chain_id(),
            data,
            nonce,
            signature: RwLock::new(None),
        };

        trace!("Built transaction: {:?}", transaction);

        Ok(transaction)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TransactionParser {
    contract: String,
    method: String,
}

impl TransactionParser {
    pub fn new() -> TransactionParser {
        TransactionParser::default()
    }

    pub fn set_contract(mut self, contract: &str) -> TransactionParser {
        self.contract = contract.to_string();

        trace!(
            "Set contract: {} to TransactionParser: {:?}",
            contract,
            self
        );

        self
    }

    pub fn set_method(mut self, method: &str) -> TransactionParser {
        self.method = method.to_string();

        trace!("Set method: {} to TransactionParser: {:?}", method, self);

        self
    }

    pub fn parse<T: TryFrom<ContractOutput, Error = VdrError>>(
        self,
        client: &LedgerClient,
        bytes: &[u8],
    ) -> VdrResult<T> {
        if bytes.is_empty() {
            let vdr_error = VdrError::ContractInvalidResponseData {
                msg: "Empty response bytes".to_string(),
            };

            warn!("Error: {:?} during transaction output parse", vdr_error);

            return Err(vdr_error);
        }
        let contract = client.contract(&self.contract)?;
        let output = contract.decode_output(&self.method, bytes)?;

        if output.is_empty() {
            let vdr_error = VdrError::ContractInvalidResponseData {
                msg: "Unable to parse response".to_string(),
            };

            warn!("Error: {:?} during transaction output parse", vdr_error);

            return Err(vdr_error);
        }

        trace!("Decoded transaction output: {:?}", output);

        T::try_from(output)
    }
}