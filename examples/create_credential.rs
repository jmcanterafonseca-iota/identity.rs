// Creates self-signed credential

mod common;

use identity::iota::Client;
use identity::core::json;
use identity::core::FromJson;
use identity::core::ToJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::CredentialBuilder;
use identity::credential::Subject;
use identity::credential::VerifiableCredential;
use identity::crypto::KeyPair;
use identity::iota::Document;
use identity::iota::Result;
use identity::core::encode_b58;
use std::env;

fn build_credential(issuer: &Document, subject: &Document) -> Result<Credential> {
  let subject: Subject = Subject::from_json_value(json!({
    "id": subject.id().as_str(),
    "gs1CompanyPrefix": {
      "type": "identifier",
      "value": "9544444"
    }
  }))?;

  let credential: Credential = CredentialBuilder::default()
    .issuer(Url::parse(issuer.id().as_str())?)
    .type_("GlobalIdentifierCredential")
    .subject(subject)
    .build()?;

  Ok(credential)
}

#[smol_potat::main]
async fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  /*
  if args.len() < 3 {
    panic!("Please provide a DID and a Private Key");
  }
  */
  let client: Client = Client::new()?;

  // Create a DID Document/KeyPair for the credential issuer.
  let (doc_iss, key_iss): (Document, KeyPair) = common::document(&client).await?;

  println!("Public Key > {}", encode_b58(key_iss.public()));
  println!("Private Key > {}", encode_b58(key_iss.secret()));

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let credential: Credential = build_credential(&doc_iss, &doc_iss)?;

  // Sign the Credential with the issuer secret key - the result is a VerifiableCredential.
  let vc: VerifiableCredential = credential.sign(&doc_iss, "#authentication".into(), key_iss.secret())?;

  println!("Credential > {:#}", vc.to_json()?);

  Ok(())
}
