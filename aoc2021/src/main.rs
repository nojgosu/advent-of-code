use crate::sonar_sweep::sonar_sweep_solution;
use crate::dive::dive_solution;
use crate::binary_diagnostic::binary_diagnostic_solution;

mod sonar_sweep;
mod dive;
mod binary_diagnostic;

fn main() {
    sonar_sweep_solution();
    dive_solution();
    binary_diagnostic_solution();

}





