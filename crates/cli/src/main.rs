#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
use strength_calc_core::{SerialAssembler, prelude::*};
use strength_calc_elements::Bar2;
use strength_calc_materials::Steel;
use strength_calc_solver::solve;

#[derive(Parser)]
#[command(author, version, about = "Strength‑Calc CLI demo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a 1D bar demo problem.
    Demo,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Demo => demo(),
    }
}

fn demo() {
    // ----- Model definition -----
    let steel = Steel::default();
    let length = 1.0_f64; // m

    let elem = Bar2 {
        node_ids: [0, 1],
        e: steel.e(),
        area: steel.area().unwrap(),
        length,
    };

    let elements: Vec<Box<dyn Element>> = vec![Box::new(elem)];
    let dof = 2; // 1 DOF per node × 2 nodes

    // ----- Assembly -----
    let assembler = SerialAssembler;
    let (k, mut f) = assembler.assemble(dof, &elements);

    // Apply axial force −1000 N at node 1.
    f[1] = -1.0e3;

    // Fix node 0 (Dirichlet BC).
    let u = solve(k, f, &[0]);

    // ----- Post‑processing -----
    let stress = steel.e() * (u[1] - u[0]) / length;
    println!("Displacement at node 1: {:.6e} m", u[1]);
    println!("Axial stress: {:.6e} Pa", stress);
}
