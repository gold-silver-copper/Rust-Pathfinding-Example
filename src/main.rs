use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32, i32); // 3D position with (x, y, z)

impl Pos {
    fn successors(&self) -> Vec<(Pos, usize)> {
        let &Pos(x, y, z) = self;
        vec![
            Pos(x + 1, y + 2, z), 
            Pos(x + 5  , y, z),
            Pos(x + 1, y + 2, z+3), 
        ]
        .into_iter()
        .map(|p| (p, 1))
        .collect()
    }
}

fn main() {
    static GOAL: Pos = Pos(40, 80, 0); // Define the 3D goal position
    static GOAL2: Pos = Pos(40, 0, 0); // Define the 3D goal position
    static GOAL3: Pos = Pos(40, 80, 120); // Define the 3D goal position
    let result = dijkstra(&Pos(0, 0, 0), |p| p.successors(), |p| *p == GOAL); // result increments are in factors of 5,0,0 because that is the fastest vector to use to reach the goal from (0,0,0) to (40,0,0)
    let result2 = dijkstra(&Pos(0, 0, 0), |p| p.successors(), |p| *p == GOAL2); // result increments are in factors of 1,2,0 because that is the only vector to use to reach the goal from (0,0,0) to (40,80,0)
    let result3 = dijkstra(&Pos(0, 0, 0), |p| p.successors(), |p| *p == GOAL3); // result increments are in factors of 1,2,3 because that is the only vector to use to reach the goal from (0,0,0) to (40,80,120)

    println!("{:?}", result);
    println!("{:?}", result2);
    println!("{:?}", result3);
   // assert_eq!(result.expect("no path found").1, 4);
}