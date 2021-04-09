// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::Url;
use identity::crypto::KeyPair;
use identity::did::Service;
use identity::did::DID;
use identity::iota::Client;
use identity::iota::Document;
use identity::iota::Network;
use identity::iota::Result;
use identity::iota::TangleRef;

// A helper function to generate and new DID Document/KeyPair, sign the
// document, publish it to the Tangle, and return the Document/KeyPair.
pub async fn document(client: &Client) -> Result<(Document, KeyPair)> {
  // Generate a new DID Document and public/private key pair.
  //
  // The generated document will have an authentication key associated with
  // the keypair.
  let keypair: KeyPair = KeyPair::new_ed25519()?;
  let mut document: Document = Document::from_keypair(&keypair)?;

  println!("{}", document.id().as_str());

  let mut service_id_str = document.id().as_str().to_owned();
  service_id_str.push_str("#product-authenticity");

  let service_id: DID = service_id_str.parse().unwrap();
  let service_url: Url = "http://localhost:9001/authenticity".parse().unwrap();

  let service: Service<()> = Service::builder(())
  .id(service_id)
  .type_("ProductAuthenticity")
  .service_endpoint(service_url)
  .build()
  .unwrap();

  // Now service is added to document
  // Get a mutable reference to the **core** DID Document
  // This is unsafe because you completely skip the IOTA method spec. validation
  let core: &mut identity::did::Document<_, _, _> = unsafe { document.as_document_mut() };
  // Append a service to the ordered set
  // The actual stored type is DIDKey<Service> so we use .into() for conversion
  core.service_mut().append(service.into());

  document.sign(keypair.secret())?;
  println!("DID Document (signed) > {:#}", document);
  println!();

  document.publish(client).await?;

  let network: Network = document.id().into();
  let explore: String = format!("{}/transaction/{}", network.explorer_url(), document.message_id());

  println!("DID Document Transaction > {}", explore);
  println!();

  Ok((document, keypair))
}
