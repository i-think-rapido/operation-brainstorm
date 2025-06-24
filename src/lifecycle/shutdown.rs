use tracing::info;

use crate::graphics::state::State;


pub fn shutdown(state: &mut State) {
    info!("Shutting down with state: {}", state.data);
    // Simulate cleanup
    state.data.clear();
    info!("State cleared.");
}
mod lifecycle {

 



}

