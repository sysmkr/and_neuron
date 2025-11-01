#[derive(Debug)]
struct Neuron {
    w_one: f64,
    w_two: f64,
    bias: f64,
}
#[derive(Debug)]
struct Case {
    one: f64,
    two: f64,
    expected: f64,
}

impl Case {
    fn new(one: f64, two: f64, expected: f64) -> Self {
        Self { one, two, expected }
    }
}

fn step(x: f64) -> f64 {
    if x > 0.0 { 1.0 } else { 0.0 }
}

fn main() {

    let truth_table: Vec<Case> = vec![
        Case::new(0.0, 0.0, 0.0),
        Case::new(0.0, 1.0, 0.0),
        Case::new(1.0, 0.0, 0.0),
        Case::new(1.0, 1.0, 1.0),
    ];

    let mut new_neuron = Neuron { 
        w_one: 0.0, 
        w_two: 0.0, 
        bias: 0.0 
    };

    let learning_rate = 0.1;
    let num_of_cycle: usize = 5;
    let mut current_cycle: usize = 0;
    let num_of_case: usize = 4;

    while current_cycle < num_of_cycle {

        let mut current_case: usize = 0;
        while current_case < num_of_case {
            let sum = new_neuron.w_one * truth_table[current_case].one
                + new_neuron.w_two * truth_table[current_case].two
                + new_neuron.bias;
            let y = step(sum);
            let error = truth_table[current_case].expected - y;

            new_neuron.w_one += learning_rate * error * truth_table[current_case].one;
            new_neuron.w_two += learning_rate * error * truth_table[current_case].two;
            new_neuron.bias += learning_rate * error;

            current_case += 1;
        }
        current_cycle += 1;
    }

    println!("Trained neuron : {:?}", new_neuron);

    for case in truth_table {
        let sum = new_neuron.w_one * case.one
            + new_neuron.w_two * case.two
            + new_neuron.bias;
        let result = step(sum);
        println!("{:?} => {}", case, result);
    }
}
