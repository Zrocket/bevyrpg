use bevy::log::error;
use bevy::prelude::*;

pub fn error_pipe(In(result): In<Result<(), anyhow::Error>>) {
    if let Err(err) = result {
        error!("Error: {err:?}")
    }
}
