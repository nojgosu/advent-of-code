use crate::sonar_sweep::sonar_sweep_solution;
use crate::dive::dive_solution;
use crate::binary_diagnostic::binary_diagnostic_solution;
use crate::giant_squid::giant_squid_solution;

mod sonar_sweep;
mod dive;
mod binary_diagnostic;
mod giant_squid;

fn main() {
    sonar_sweep_solution();
    dive_solution();
    binary_diagnostic_solution();
    giant_squid_solution();

}





