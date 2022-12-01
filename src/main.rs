
use getevidence::get_evidence_service_client::GetEvidenceServiceClient;
use getevidence::GetEvidenceRequest;

pub mod getevidence {
    tonic::include_proto!("getevidence");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GetEvidenceServiceClient::connect("http://127.0.0.1:50002").await?;

    let request = tonic::Request::new(GetEvidenceRequest {
        nonce:"123456".try_into().unwrap(),
        container_id:"1".to_owned()
    });

    let response = client.get_evidence(request).await?;

    println!("RESPONSE={:?}", response.get_ref().evidence);

    Ok(())
}
