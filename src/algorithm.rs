use pathfinding::kuhn_munkres::kuhn_munkres;


fn process_hungarian() -> (i32 , Vec<usize>){
    let weights = &[
        [100, 110, 90],
        [95, 130, 75],
        [95, 140, 65],
    ];
    let (total_weight, assignment) = kuhn_munkres(weights);
    (total_weight, assignment)
}