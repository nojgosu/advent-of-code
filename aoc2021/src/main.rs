use crate::sonar_sweep::sonar_sweep_solution;
use crate::dive::dive_solution;
use crate::binary_diagnostic::binary_diagnostic_solution;
use crate::giant_squid::giant_squid_solution;
use crate::hydrothermal_venture::hydrothermal_venture_solution;
use crate::lanternfish::lanternfish_solution;
use crate::treachery_of_whales::treachery_of_whales_solution;
use crate::seven_segment_search::seven_segment_search_solution;

mod sonar_sweep;
mod dive;
mod binary_diagnostic;
mod giant_squid;
mod hydrothermal_venture;
mod lanternfish;
mod treachery_of_whales;
mod seven_segment_search;

fn main() {
    println!("*** Advent of Code 2021 ***");
    sonar_sweep_solution();
    dive_solution();
    binary_diagnostic_solution();
    giant_squid_solution();
    hydrothermal_venture_solution();
    lanternfish_solution();
    treachery_of_whales_solution();
    seven_segment_search_solution();

}





