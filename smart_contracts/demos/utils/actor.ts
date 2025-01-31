import {
  RoleControl,
  SchemaRegistry,
  CredentialDefinitionRegistry,
  ValidatorControl,
  UpgradeControl,
  EthereumExtDidRegistry,
} from '../../contracts-ts'
import { Account, AccountInfo } from '../../utils'

export class Actor {
  public account: Account
  public roleControl!: RoleControl
  public validatorControl!: ValidatorControl
  public ethereumDIDRegistry!: EthereumExtDidRegistry
  public schemaRegistry!: SchemaRegistry
  public credentialDefinitionRegistry!: CredentialDefinitionRegistry
  public upgradeControl!: UpgradeControl

  constructor(accountInfo?: AccountInfo) {
    this.account = accountInfo ? new Account(accountInfo) : new Account()
  }

  public async init() {
    this.roleControl = await new RoleControl(this.account).getInstance(RoleControl.defaultAddress)
    this.validatorControl = await new ValidatorControl(this.account).getInstance(ValidatorControl.defaultAddress)
    this.ethereumDIDRegistry = await new EthereumExtDidRegistry(this.account).getInstance(
      EthereumExtDidRegistry.defaultAddress,
    )
    this.schemaRegistry = await new SchemaRegistry(this.account).getInstance(SchemaRegistry.defaultAddress)
    this.credentialDefinitionRegistry = await new CredentialDefinitionRegistry(this.account).getInstance(
      CredentialDefinitionRegistry.defaultAddress,
    )
    this.upgradeControl = await new UpgradeControl(this.account).getInstance(UpgradeControl.defaultAddress)
    return this
  }

  public get address() {
    return this.account.address
  }

  public get did() {
    return this.account.did
  }

  public get didEthr() {
    return this.account.didEthr
  }

  public get didDocument() {
    return this.account.didDocument
  }
}
