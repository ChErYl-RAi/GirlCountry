
//this is just copied from the crate docs LMAO
use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(i32, i32);

impl Pos {
  fn distance(&self, other: &Pos) -> u32 {
    (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
  }

  fn successors(&self) -> Vec<(Pos, u32)> {
    let &Pos(x, y) = self;
    vec![
        (Pos(x+1, y), 1),
        (Pos(x-1, y), 1),
        (Pos(x, y+1), 1),
        (Pos(x, y-1), 1),
        (Pos(x+1, y+1), 1),
        (Pos(x-1, y+1), 1),
        (Pos(x+1, y-1), 1),
        (Pos(x-1, y-1), 1)
    ]
}
}

pub fn find(xpos: i32, ypos: i32, xdir: i32, ydir: i32) -> Vec<i32> {
    let goal = Pos(xdir, ydir);

    let result = astar(
        &Pos(xpos, ypos),
        |p| p.successors(),
        |p| p.distance(&goal),
        |p| *p == goal,
    );

    let path = result.expect("no path found").0;

    if path.len() < 2 {
        return vec![xpos, ypos];
    }

    vec![path[1].0, path[1].1]
}