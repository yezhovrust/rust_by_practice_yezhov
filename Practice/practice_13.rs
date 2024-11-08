use rand::Rng;

fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();


    if total % n as u32 != 0 {
        return 0;
    }

    let average = total / n as u32;
    let mut moves = 0;
    let mut balance = 0;


    for &shipment in shipments.iter() {

        balance += shipment as i32 - average as i32;


        moves += balance.abs();
    }

    moves as usize
}

fn can_distribute_evenly(shipments: &Vec<u32>) -> bool {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();

    total % n as u32 == 0
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let average = 10;
    let mut shipments = vec![average; n];

    let total: u32 = shipments.iter().sum();
    let adjustment = total % n as u32;

    for i in 0..adjustment as usize {
        shipments[i] += 1;
    }

    shipments
}

fn main() {

    let shipments = vec![8, 2, 2, 4, 4];
    let result = count_permutation(&shipments);
    println!("Minimum number of moves: {}", result);

    let shipments = vec![9, 3, 7, 2, 9];
    let result = count_permutation(&shipments);
    println!("Minimum number of moves: {}", result);


    let possible = can_distribute_evenly(&shipments);
    println!("Can distribute evenly: {}", possible);


    let generated_shipments = gen_shipments(5);
    println!("Generated shipments: {:?}", generated_shipments);
}
