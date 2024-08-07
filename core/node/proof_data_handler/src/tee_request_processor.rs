use std::sync::Arc;

use axum::{extract::Path, Json};
use zksync_config::configs::ProofDataHandlerConfig;
use zksync_dal::{ConnectionPool, Core, CoreDal};
use zksync_object_store::ObjectStore;
use zksync_prover_interface::{
    api::{
        RegisterTeeAttestationRequest, RegisterTeeAttestationResponse, SubmitProofResponse,
        SubmitTeeProofRequest, TeeProofGenerationDataRequest, TeeProofGenerationDataResponse,
    },
    inputs::TeeVerifierInput,
};
use zksync_types::L1BatchNumber;

use crate::errors::RequestProcessorError;

#[derive(Clone)]
pub(crate) struct TeeRequestProcessor {
    blob_store: Arc<dyn ObjectStore>,
    pool: ConnectionPool<Core>,
    config: ProofDataHandlerConfig,
}

impl TeeRequestProcessor {
    pub(crate) fn new(
        blob_store: Arc<dyn ObjectStore>,
        pool: ConnectionPool<Core>,
        config: ProofDataHandlerConfig,
    ) -> Self {
        Self {
            blob_store,
            pool,
            config,
        }
    }

    pub(crate) async fn get_proof_generation_data(
        &self,
        request: Json<TeeProofGenerationDataRequest>,
    ) -> Result<Json<TeeProofGenerationDataResponse>, RequestProcessorError> {
        tracing::info!("Received request for proof generation data: {:?}", request);

        let mut connection = self
            .pool
            .connection()
            .await
            .map_err(RequestProcessorError::Dal)?;

        let l1_batch_number_result = connection
            .tee_proof_generation_dal()
            .get_next_block_to_be_proven(self.config.proof_generation_timeout())
            .await
            .map_err(RequestProcessorError::Dal)?;
        let l1_batch_number = match l1_batch_number_result {
            Some(number) => number,
            None => return Ok(Json(TeeProofGenerationDataResponse(None))),
        };

        let tee_verifier_input: TeeVerifierInput = self
            .blob_store
            .get(l1_batch_number)
            .await
            .map_err(RequestProcessorError::ObjectStore)?;

        Ok(Json(TeeProofGenerationDataResponse(Some(Box::new(
            tee_verifier_input,
        )))))
    }

    pub(crate) async fn submit_proof(
        &self,
        Path(l1_batch_number): Path<u32>,
        Json(proof): Json<SubmitTeeProofRequest>,
    ) -> Result<Json<SubmitProofResponse>, RequestProcessorError> {
        let l1_batch_number = L1BatchNumber(l1_batch_number);
        let mut connection = self
            .pool
            .connection()
            .await
            .map_err(RequestProcessorError::Dal)?;
        let mut dal = connection.tee_proof_generation_dal();

        tracing::info!(
            "Received proof {:?} for block number: {:?}",
            proof,
            l1_batch_number
        );
        dal.save_proof_artifacts_metadata(
            l1_batch_number,
            &proof.0.signature,
            &proof.0.pubkey,
            &proof.0.proof,
            proof.0.tee_type,
        )
        .await
        .map_err(RequestProcessorError::Dal)?;

        Ok(Json(SubmitProofResponse::Success))
    }

    pub(crate) async fn register_tee_attestation(
        &self,
        Json(payload): Json<RegisterTeeAttestationRequest>,
    ) -> Result<Json<RegisterTeeAttestationResponse>, RequestProcessorError> {
        tracing::info!("Received attestation: {:?}", payload);

        let mut connection = self
            .pool
            .connection()
            .await
            .map_err(RequestProcessorError::Dal)?;
        let mut dal = connection.tee_proof_generation_dal();

        dal.save_attestation(&payload.pubkey, &payload.attestation)
            .await
            .map_err(RequestProcessorError::Dal)?;

        Ok(Json(RegisterTeeAttestationResponse::Success))
    }
}
