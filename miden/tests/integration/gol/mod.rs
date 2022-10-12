use miden::{Assembler, ProgramInputs};
use processor::Process;
use proptest::prelude::*;
use std::fs;

proptest! { 
    #[test]
    fn gol_test(test_values in prop::collection::vec(any::<bool>(), 16), n in 0_usize..5) {
        let path = "tests/integration/gol/gol.masm";
        let source = fs::read_to_string(path).expect("Could not read source file");
        let assembler = Assembler::new(true);
        let program = assembler
            .compile(source.as_str()).expect("Could not compile source");
        let mut inputs: Vec<u64> = test_values.iter().map(|v| u64::from(*v)).collect();
        for _ in 0..n {
            // run it in miden assembly
            let program_inputs = ProgramInputs::from_stack_inputs(&inputs).expect("could not create program inputs");
            let mut process = Process::new(program_inputs.clone());
            let program_outputs = process.execute(&program).expect("could not execute program");
            // run it in rust
            let expected_outputs = to_1d(gol(&to_2d(inputs, 4)));
            // compare
            assert_eq!(program_outputs.stack(), expected_outputs);
            inputs = program_outputs.stack().to_vec();
        }
        
    }
}

// Source: https://dev.to/dineshgdk/game-of-life-in-rust-4mfc
// function to compute the next generation
fn gol(grid: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {

    // get the number of rows
    let n = grid.len();

    // get the number of columns
    let m = grid[0].len();

    // create an empty grid to compute the future generation
    let mut future: Vec<Vec<i8>> = vec![vec![0; n]; m];

    // iterate through each and every cell
    for i in 0..n {
        for j in 0..m {

            // the current state of the cell (alive / dead)
            let cell_state = grid[i][j];

            // variable to track the number of alive neighbors
            let mut live_neighbors = 0;

            // iterate through every neighbors including the current cell
            for x in -1i8..=1 {
                for y in -1i8..=1 {

                    // position of one of the neighbors (new_x, new_y)
                    let new_x = (i as i8) + x;
                    let new_y = (j as i8) + y;

                    // make sure the position is within the bounds of the grid
                    if new_x > 0 && new_y > 0 && new_x < n as i8 && new_y < m as i8 {
                        live_neighbors += grid[new_x as usize][new_y as usize];
                    }
                }
            }

            // substract the state of the current cell to get the number of alive neighbors
            live_neighbors -= cell_state;

            // applying the rules of game of life to get the future generation
            if cell_state == 1 && live_neighbors < 2 {
                future[i][j] = 0;
            } else if cell_state == 1 && live_neighbors > 3 {
                future[i][j] = 0;
            } else if cell_state == 0 && live_neighbors == 3 {
                future[i][j] = 1;
            } else {
                future[i][j] = cell_state;
            }
        }
    }

    // return the future generation
    future
}

fn to_1d(matrix: Vec<Vec<i8>>) -> Vec<u64> {
    matrix.iter().flat_map(|arr| arr.iter().map(|v| u64::try_from(*v).expect("Could not convert i8 to u64"))).collect()
}

fn to_2d(inputs: Vec<u64>, rows: usize) -> Vec<Vec<i8>> {
    (0..rows).into_iter()
        .map(|row| {
            (0..inputs.len()/rows)
                .map(|col| i8::try_from(inputs[row * inputs.len()/rows + col]).expect("Could not convert u64 to i8"))
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>()
}