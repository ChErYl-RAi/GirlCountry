use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
  fn successors(&self) -> Vec<Pos> {
    let &Pos(x, y) = self;
    vec![Pos(x+1,y+1), Pos(x-1,y-1), Pos(x,y+1), Pos(x,y-1)]
  }
}


fn find(xpos: i32, ypos: i32, xdir: i32, ydir: i32){

    let GOAL: Pos = Pos(xdir, ydir);
    let result = bfs(&Pos(xpos, ypos), |p| p.successors(), |p| *p == GOAL);
    assert_eq!(result.expect("no path found").len(), 5);

}