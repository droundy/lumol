//! Molecular dynamics simulation of an Argon crystal melt.
//!
//! In this example, we do everything by hand, from the system setup to the
//! simulation run.
extern crate cymbalum;
use cymbalum::*;

fn main() {
    Logger::stdout();
    let mut system = System::from_cell(UnitCell::cubic(17.0));

    // Create a cubic crystal of Argon by hand.
    for i in 0..5 {
        for j in 0..5 {
            for k in 0..5 {
                let mut part = Particle::new("Ar");
                part.position = Vector3D::new(
                        i as f64 * 3.4,
                        j as f64 * 3.4,
                        k as f64 * 3.4
                );
                system.add_particle(part);
            }
        }
    }
    system.add_pair_interaction("Ar", "Ar", Box::new(LennardJones{
                                                sigma: units::from(3.4, "A").unwrap(),
                                                epsilon: units::from(1.0, "kJ/mol").unwrap()}));

    let mut velocities = BoltzmanVelocities::new(units::from(300.0, "K").unwrap());
    velocities.seed(129);
    velocities.init(&mut system);

    let mut simulation = Simulation::new(MolecularDynamics::new(units::from(1.0, "fs").unwrap()));
    // Write the trajectory to `trajectory.xyz` every 10 steps
    simulation.add_output_with_frequency(TrajectoryOutput::new("trajectory.xyz").unwrap(), 10);
    // Write the energy to `energy.dat` every step
    simulation.add_output(EnergyOutput::new("energy.dat").unwrap());

    simulation.run(&mut system, 5000);

    println!("All done!")
}
