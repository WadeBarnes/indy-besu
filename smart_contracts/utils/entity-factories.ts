export function createBaseDidDocument(did: string) {
  const kid = `${did}#KEY-1`
  return JSON.stringify({
    '@context': ['https://www.w3.org/ns/did/v1'],
    id: did,
    controller: [did],
    verificationMethod: [
      {
        id: kid,
        type: 'Ed25519VerificationKey2018',
        controller: did,
        publicKeyMultibase: 'zAKJP3f7BD6W4iWEQ9jwndVTCBq8ua2Utt8EEjJ6Vxsf',
      },
    ],
    authentication: [kid],
    assertionMethod: [],
    capabilityInvocation: [],
    capabilityDelegation: [],
    keyAgreement: [],
    service: [],
    alsoKnownAs: [],
  })
}

interface CreateSchemaParams {
  issuer: string
  name?: string
  version?: string
  attrNames?: string[]
}

export function createSchemaObject({
  issuer,
  name = 'BasicIdentity',
  version = '1.0.0',
  attrNames = ['First Name', 'Last Name'],
}: CreateSchemaParams) {
  const issuerId = `did:ethr:${issuer}`
  const id = `${issuerId}/anoncreds/v0/SCHEMA/${name}/${version}`
  return {
    id,
    schema: JSON.stringify({
      id,
      issuerId,
      name,
      version,
      attrNames,
    }),
  }
}

interface CreateCredentialDefinitionParams {
  issuer: string
  schemaId: string
  credDefType?: string
  tag?: string
  value?: Record<string, string>
}

export function createCredentialDefinitionObject({
  issuer,
  schemaId,
  credDefType = 'CL',
  tag = 'BasicIdentity',
  value = {
    n: '779...397',
    rctxt: '774...977',
    s: '750..893',
    z: '632...005',
  },
}: CreateCredentialDefinitionParams) {
  const issuerId = `did:ethr:${issuer}`
  const id = `${issuerId}/anoncreds/v0/CLAIM_DEF/${schemaId}/${tag}`
  return {
    id,
    credDef: JSON.stringify({
      id,
      issuerId,
      schemaId,
      credDefType,
      tag,
      value,
    }),
  }
}
