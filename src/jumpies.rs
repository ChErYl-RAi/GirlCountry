
//this is just copied from the crate docs LMAO
use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(i32, i32);

use crate::TileType;
use crate::Tile;

impl Pos {
  fn distance(&self, other: &Pos) -> u32 {
    (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
  }

  fn successors(&self,map:&Vec<Vec<Tile>>) -> Vec<(Pos, u32)> {
    let &Pos(x, y) = self;
    vec![
        (Pos(x+1, y-1), 1*(calculate_wall(map,x,y) as u32)),
        (Pos(x-1, y+1), 1*(calculate_wall(map,x,y) as u32)),
        (Pos(x, y+1), 1*(calculate_wall(map,x,y) as u32)),
        (Pos(x, y-1), 1*(calculate_wall(map,x,y) as u32)),
        //(Pos(x+1, y+1), 1*(calculate_wall(map,x,y) as u32)),
        //(Pos(x-1, y+1), 1*(calculate_wall(map,x,y) as u32)),
        //(Pos(x+1, y-1), 1*(calculate_wall(map,x,y) as u32)),
        //(Pos(x-1, y-1), 1*(calculate_wall(map,x,y) as u32))
    ]
}
}

pub fn find(xpos: i32, ypos: i32, xdir: i32, ydir: i32, map:&Vec<Vec<Tile>>) -> Vec<i32> {
    let goal = Pos(xdir, ydir);

    let result = astar(
        &Pos(xpos, ypos),
        |p| p.successors(&map),
        |p| p.distance(&goal),
        |p| *p == goal,
    );

    let path = result.clone().expect("no path found").0;
    let kpk = result.expect("no path found").1;

    if path.len() < 2 {
        return vec![xpos, ypos];
    }
    if kpk >999{
        return vec![xpos, ypos];
    } else{
        return vec![path[1].0, path[1].1]
    }
}

fn calculate_wall(map:&Vec<Vec<Tile>>, x:i32, y:i32) -> i32{
    if x >49 || y >49 || x<0 || y<0{
        return 99999;
    }else {
        if map[x as usize][y as usize].tile == TileType::Road
        {
            return 1;
        }
        else if map[x as usize][y as usize].tile == TileType::Grass || map[x as usize][y as usize].tile == TileType::Bridge{
            return 2;
        } else {
            return 9999;
        }
    }
}