use lambda_runtime as lambda;
use lambda::{error::HandlerError, lambda};

use log::{self, error};
use simple_logger::{self};

use std::error::Error;

use ruptane_common::{SignedPayload, VehicleUpdateRequest, VehicleUpdateResponse, Signature};

const LAMBDA_VERSION: u32 = 7;


fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(wrapper: SignedPayload<VehicleUpdateRequest>,
              c: lambda::Context) -> Result<SignedPayload<VehicleUpdateResponse>, HandlerError> {
    if wrapper.signatures.is_empty() {
        error!("No signature {}", c.aws_request_id);
        return Err(c.new_error("No signature in request"));
    }

    //TODO verify signature
    let e = wrapper.signed;

    //TODO do a lookup for this vehicle and generate a response
    let raw_resp = VehicleUpdateResponse {
        resp_version: format!("{:04}",LAMBDA_VERSION),
        vehicle_id: e.vehicle_id,
        message: format!("Hello !"),
    };

    //TODO generate non-canned signatures
    let sig = Signature {
        keyid: "9a406d99e362e7c93e7acfe1e4d6585221315be817f350c026bbee84ada260da".into(),
        method: "ed25519".into(),
        sig: "335272f77357dc0e9f1b74d72eb500e4ff0f443f824b83405e2b21264778d1610e0a5f2663b90eda8ab05a28b5b64fc15514020985d8a93576fe33b287e1380f".into()
    };
    let wrapped_resp = SignedPayload::<VehicleUpdateResponse> {
        signatures: vec![sig],
        signed: raw_resp
    };

    Ok(wrapped_resp)
}