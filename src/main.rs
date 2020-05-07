use lambda_runtime as lambda;
use lambda::{error::HandlerError, lambda};

use log::{self, error};
use simple_logger::{self};

use std::error::Error;

use ruptane_common::{VehicleUpdateRequest, VehicleUpdateResponse};

const LAMBDA_VERSION: u32 = 6;


fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: VehicleUpdateRequest, c: lambda::Context) -> Result<VehicleUpdateResponse, HandlerError> {
    if e.vehicle_id == "" {
        error!("No vehicle_id {}", c.aws_request_id);
        return Err(c.new_error("No vehicle_id in request"));
    }

    Ok(VehicleUpdateResponse {
        resp_version: format!("{:04}",LAMBDA_VERSION),
        vehicle_id: e.vehicle_id,
        message: format!("Hello !"),
    })
}