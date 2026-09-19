use noise::{Perlin, Vector2, core::perlin::perlin_2d, permutationtable::PermutationTable};
use raylib::{ffi::{CSSPalette, RaylibPalette}, prelude::*};
use rand;
use std::{i32, thread};

mod synth;
mod jumpies;

#[derive(PartialEq, Clone, Copy)]
enum TileType {
    Grass,
    Ball,
    A,
    Factory,
    House,
    River,
    Bridge,
    Tree,
    Rock,
    Bush,
    Wall,
    Sculpture,
    Road,
}


#[derive(PartialEq)]
enum BrushType {
    Tile,
    Rise,
    Depress,
    Girl,
}

struct Tile{
    tile:TileType,
    height:i32,
    progress:i32,
}

#[derive(PartialEq,Clone)]
enum GirlModes{
    Idle,
    Going,
    Interact,
    Work,
    Attent,
}

#[derive(Clone)]
struct Girl{
    x:i32,
    y:i32,
    rot:i32,
    col:bool,
    mode: GirlModes,
    cooldown:f32,
    destination:Vec<i32>,
    money:i32,
}

fn main(){

    let (mut rl, thread) = raylib::init()
        .size(768, 640)
        .title("GirlCountry")
        .build();
    rl.set_target_fps(60);
    let grass_sprite = rl.load_texture(&thread, "assets/grass.png").unwrap();
    let a_sprite = rl.load_texture(&thread, "assets/a.png").unwrap();
    let factory_sprite = rl.load_texture(&thread, "assets/factory.png").unwrap();
    let house_sprite = rl.load_texture(&thread, "assets/house.png").unwrap();
    let ball_sprite = rl.load_texture(&thread, "assets/ball.png").unwrap();
    let river_sprite = rl.load_texture(&thread, "assets/water.png").unwrap();
    let bridge_sprite = rl.load_texture(&thread, "assets/bridge.png").unwrap();
    let cursor = rl.load_texture(&thread, "assets/select.png").unwrap();
    let talk_sprite = rl.load_texture(&thread, "assets/talk.png").unwrap();
    let work_sprite = rl.load_texture(&thread, "assets/work.png").unwrap();
    let tree_sprite = rl.load_texture(&thread, "assets/tree.png").unwrap();
    let bush_sprite = rl.load_texture(&thread, "assets/bush.png").unwrap();
    let wall_sprite = rl.load_texture(&thread, "assets/wall.png").unwrap();
    let sculpture_sprite = rl.load_texture(&thread, "assets/sculpture.png").unwrap();
    let rock_sprite = rl.load_texture(&thread, "assets/stone.png").unwrap();
    let alert_sprite = rl.load_texture(&thread, "assets/alert.png").unwrap();
    let go_sprite = rl.load_texture(&thread, "assets/mitawa.png").unwrap();
    let road_sprite = rl.load_texture(&thread, "assets/path.png").unwrap();

    let font = [
        rl.load_font_ex(&thread, "assets/ipam.ttf", 17, None).unwrap(),
        rl.load_font_ex(&thread, "assets/ipam.ttf", 20, None).unwrap(),
        rl.load_font_ex(&thread, "assets/ipam.ttf", 25, None).unwrap(),
        rl.load_font_ex(&thread, "assets/ipam.ttf", 50, None).unwrap()];
    // 17 20 25 50
    
    let message = "";

    let mut girls:Vec<Girl>= vec![];

    let bgirl_sprite= vec![
        rl.load_texture(&thread, "assets/charab1.png").unwrap(),
        rl.load_texture(&thread, "assets/charab2.png").unwrap(),
        rl.load_texture(&thread, "assets/charab3.png").unwrap(),
        rl.load_texture(&thread, "assets/charab4.png").unwrap(),
        rl.load_texture(&thread, "assets/charab5.png").unwrap(),
        rl.load_texture(&thread, "assets/charab6.png").unwrap(),
        rl.load_texture(&thread, "assets/charab7.png").unwrap(),
        rl.load_texture(&thread, "assets/charab8.png").unwrap(),
    ];

    let wgirl_sprite= vec![
        rl.load_texture(&thread, "assets/charaw1.png").unwrap(),
        rl.load_texture(&thread, "assets/charaw2.png").unwrap(),
        rl.load_texture(&thread, "assets/charaw3.png").unwrap(),
        rl.load_texture(&thread, "assets/charaw4.png").unwrap(),
        rl.load_texture(&thread, "assets/charaw5.png").unwrap(),
        rl.load_texture(&thread, "assets/charaw6.png").unwrap(),
        rl.load_texture(&thread, "assets/charaw7.png").unwrap(),
        rl.load_texture(&thread, "assets/charaw8.png").unwrap(),
    ];


    let building_sprite= vec![
        rl.load_texture(&thread, "assets/building1.png").unwrap(),
        rl.load_texture(&thread, "assets/building2.png").unwrap(),
        rl.load_texture(&thread, "assets/building3.png").unwrap(),
        rl.load_texture(&thread, "assets/building4.png").unwrap(),
        rl.load_texture(&thread, "assets/building5.png").unwrap(),
        rl.load_texture(&thread, "assets/building6.png").unwrap(),
        rl.load_texture(&thread, "assets/building7.png").unwrap(),
        rl.load_texture(&thread, "assets/building8.png").unwrap(),
        rl.load_texture(&thread, "assets/building9.png").unwrap(),
        rl.load_texture(&thread, "assets/building10.png").unwrap(),
        rl.load_texture(&thread, "assets/building11.png").unwrap(),
    ];

    let mut res_wood =  10;
    let mut res_food =  10;
    let mut res_stone = 10;
    let mut res_monies = 50;

    let mut bellring = false;

    let mut bells =thread::spawn(move || {
        synth::mynothing();
    });
    
    let mut posx: f32 = rand::random_range(400..3600) as f32;
    let mut posy: f32 = rand::random_range(-30..770) as f32;
    let mut map: Vec<Vec<Tile>> = vec![];
    let mut occupiedtiles: Vec<Vec<i32>> = vec![];
    let mut paintbrush = TileType::House;
    let mut paintbrushtype = BrushType::Tile;
    let mut partners: Vec<Vec<i32>> = vec![];
    let mut plantlocs: Vec<Vec<i32>> = vec![];

    let girlcum: u32 = rand::random(); // get it because seed
    let hasher = PermutationTable::new(girlcum) ;
    let mut initialhmapvec: Vec<Vec<i32>> = vec![];
    
    for x in 0..50{
        let v=vec![];
        initialhmapvec.push(v);
        for y in 0..50{
            initialhmapvec[x].push ( 
                ((perlin_2d(Vector2::new(((x as f32)/30.0) as f64, ((y as f32)/30.0) as f64), &hasher) / 2.0 +0.5)*5.0-2.0).floor() as i32
            );
            if initialhmapvec[x][y] < 0{
                initialhmapvec[x][y] = 0;
            }
        }
    }

    

    println!("{:?}",initialhmapvec);

    for i in 0..50{
        let mut tmp: Vec<Tile> = vec![];
        for j in 0..50{
            if i <23 || i>27{
                
                    if rand::random_bool(0.1){
                        if j == 48-i{
                            tmp.push(
                                Tile{
                                    tile: TileType::Road,
                                    height: initialhmapvec[i as usize][j as usize],
                                    progress:12
                                }
                            );
                        }else{
                            if rand::random_bool(0.33){
                                tmp.push(
                                    Tile{
                                        tile: TileType::Tree,
                                        height: initialhmapvec[i as usize][j as usize],
                                        progress:12
                                    }
                                );
                                plantlocs.push(vec![i,j]);
                            }else if rand::random_bool(0.5){
                                tmp.push(
                                    Tile{
                                        tile: TileType::Bush,
                                        height: initialhmapvec[i as usize][j as usize],
                                        progress:12
                                    }
                                );
                                plantlocs.push(vec![i,j]);
                            } else {
                                tmp.push(
                                    Tile{
                                        tile: TileType::Rock,
                                        height: initialhmapvec[i as usize][j as usize],
                                        progress:12
                                    }
                                );
                                plantlocs.push(vec![i,j]);
                            }
                        }
                    } else {
                        if j == 48-i{
                            tmp.push(
                                Tile{
                                    tile: TileType::Road,
                                    height: initialhmapvec[i as usize][j as usize],
                                    progress:12
                                }
                            );
                        }else{
                            tmp.push(
                                Tile{
                                    tile: TileType::Grass,
                                    height: initialhmapvec[i as usize][j as usize],
                                    progress:12
                                }
                            );
                        }
                        if rand::random_bool(0.04){
                            girls.push(
                                Girl { x: i, y: j, rot: rand::random_range(0..8), col: rand::random_bool(0.5), mode:GirlModes::Idle, cooldown:1.0,destination:vec![0,0], money:100}
                            );
                            occupiedtiles.push(vec![i,j]);
                        }
                    }
                
            } else {
                if j == 48-i || j==47-i || j==49-i{
                    tmp.push(
                        Tile{
                            tile: TileType::Bridge,
                            height: 0,
                                progress:12
                        }
                    );
                }else {
                    tmp.push(
                        Tile{
                            tile: TileType::River,
                            height: 0,
                                progress:12
                        }
                    );
                }
            }
        }
        map.push(tmp);
    }
    let bgc = Color::from_hex("011800").unwrap();
    let sky = Color::from_hex("a1afd4").unwrap();



    let mut abuildtimer=0.0;
    let mut bbuildtimer=0.0;
    let mut cbuildtimer=0.0;
    let mut dbuildtimer=0.0;
    let mut progress1= false;
    let mut progress2= false;
    let mut progress3= false;
    let mut progress4= false;

    let mut month = rand::random_range(0..13) ;
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "Mids",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];

    let mus1 =months.clone();
    let mxs1 = month.clone();

    let mut music = thread::spawn(move || {
        synth::play(mus1[mxs1]);
    });

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(sky);
        

        if d.is_key_down(KeyboardKey::KEY_UP){
            if posy>-30.0{
                posy-=252.0*d.get_frame_time();
            }
        }
        if d.is_key_down(KeyboardKey::KEY_DOWN){
            if posy<770.0{
                posy+=252.0*d.get_frame_time();
            }
        }
        if d.is_key_down(KeyboardKey::KEY_LEFT){
            if posx>0.0{
                posx-=252.0*d.get_frame_time();
            }
        }
        if d.is_key_down(KeyboardKey::KEY_RIGHT){
            if posx<4000.0{
                posx+=252.0*d.get_frame_time();
            }
        }
        
        abuildtimer+=2.0*d.get_frame_time();
        bbuildtimer+=2.0*d.get_frame_time();
        cbuildtimer+=2.0*d.get_frame_time();
        dbuildtimer+=2.0*d.get_frame_time();

        if abuildtimer < 4.0
        {
            progress1=false;
        } else {
            abuildtimer= 0.0;
            progress1=true;
        }

        if bbuildtimer < 3.0
        {
            progress2=false;
        } else {
            bbuildtimer= 0.0;
            progress2=true;
        }

        if cbuildtimer < 2.0
        {
            progress3=false;
        } else {
            cbuildtimer= 0.0;
            progress3=true;
        }

        if dbuildtimer < 1.0
        {
            progress4=false;
        } else {
            dbuildtimer= 0.0;
            progress4=true;
        }

        if music.is_finished()
        {
            if !bellring{
            bells =thread::spawn(move || {
                synth::bells();
            });
            month+=1;
            if month ==13 { month =0; } ;
            bellring=true;
            }
            if bells.is_finished(){
                music.join().unwrap();
                let mus2 =months.clone();
                let mxs2 = month.clone();
                bellring=false;
                music = thread::spawn(move || {
                    synth::play(mus2[mxs2]);
                });
            }
        }
        let prevgirl =girls.clone();
        for girl in &mut girls{
            
            if girl.mode ==GirlModes::Work{
                girl.mode=GirlModes::Idle;
            }
        }

        for y in 0..50{
            for x in 0..50{
                let mut girlnum = 0;
                for girl in &mut girls{
                    if ((x-1 == girl.x && y == girl.y-1) || (x+1 == girl.x && y == girl.y+1) || (x == girl.x && y-1 == girl.y) || (x == girl.x && y+1 == girl.y)) && map[x as usize][y as usize].progress<12 && girl.mode==GirlModes::Idle{
                        girlnum+=1;
                        girl.mode=GirlModes::Work;
                    }
                    if map[x as usize][y as usize].progress<12 && girl.mode==GirlModes::Work{
                        if (x-1 == girl.x && y == girl.y-1){
                            girl.rot=4;
                        }
                        else if (x+1 == girl.x && y == girl.y+1){
                            girl.rot=0;
                        }
                        else if (x == girl.x && y-1 == girl.y){
                            girl.rot=2;
                        }
                        else if (x == girl.x && y+1 == girl.y){
                            girl.rot=6;
                        }
                    }
                }
                if girlnum==1 && progress1{
                    if map[x as usize][y as usize].progress<12{
                        map[x as usize][y as usize].progress+=1;
                    }
                }else if girlnum==2 && progress2{
                    if map[x as usize][y as usize].progress<12{
                        map[x as usize][y as usize].progress+=1;
                    }
                }else if girlnum==3 && progress3{
                    if map[x as usize][y as usize].progress<12{
                        map[x as usize][y as usize].progress+=1;
                    }
                }else if girlnum==4 && progress4{
                    if map[x as usize][y as usize].progress<12{
                        map[x as usize][y as usize].progress+=1;
                    }
                }
        
            }
        }
        
        for girp in &mut girls{
            if girp.cooldown>0.0{
                girp.cooldown-=d.get_frame_time()
            } else {
                girp.cooldown=0.0
            }
        }
        for girl in &mut girls{
            if girl.mode == GirlModes::Going{
                if girl.cooldown <= 0.0{
                    let dest= jumpies::find(girl.x,girl.y,girl.destination[0],girl.destination[1],&map);
                    girl.x= dest[0];
                    girl.y= dest[1];
                    if girl.x == girl.destination[0] && girl.y == girl.destination[1] {
                        girl.mode = GirlModes::Idle;
                    }
                    girl.cooldown = 0.2;
                }
            }
            if girl.mode==GirlModes::Idle{
            //girl rotate
            if rand::random_bool(0.01){
                girl.rot += 1;
                if girl.rot ==-1{
                    girl.rot=7;
                } else if girl.rot ==8 {
                    girl.rot=0;
                }
            }else if rand::random_bool(0.01) {
                girl.rot -= 1;
                if girl.rot ==-1{
                    girl.rot=7;
                } else if girl.rot ==8 {
                    girl.rot=0;
                }
            }
            
            
            
            //girl move
            if rand::random_bool(0.01){
                let oldpos = vec![girl.x, girl.y];
                if girl.x>1 && girl.rot == 7 && map[(girl.x-1) as usize][girl.y as usize].tile == TileType::Grass && !occupiedtiles.contains(&vec![girl.x-1, girl.y])
                {
                    girl.x-=1;
                } else if girl.x<49 && girl.rot == 3 && map[(girl.x+1) as usize][girl.y as usize].tile == TileType::Grass && !occupiedtiles.contains(&vec![girl.x+1, girl.y])
                {
                    girl.x+=1;
                }
                if girl.y>1 && girl.rot == 6 && map[girl.x as usize][(girl.y-1) as usize].tile == TileType::Grass && !occupiedtiles.contains(&vec![girl.x, girl.y-1])
                {
                    girl.y-=1;
                } else if girl.y<49 && girl.rot == 2 && map[girl.x as usize][(girl.y+1) as usize].tile == TileType::Grass && !occupiedtiles.contains(&vec![girl.x, girl.y+1])
                {
                    girl.y+=1;
                }
                if girl.x<49 && girl.y>1 && girl.rot == 4 && map[(girl.x+1) as usize][(girl.y-1) as usize].tile == TileType::Grass && !occupiedtiles.contains(&vec![girl.x+1, girl.y-1])
                {
                    girl.x+=1;
                    girl.y-=1;
                } else if girl.y<49 && girl.x>1 && girl.rot == 0 && map[(girl.x-1) as usize][(girl.y+1) as usize].tile == TileType::Grass && !occupiedtiles.contains(&vec![girl.x-1, girl.y+1])
                {
                    girl.x-=1;
                    girl.y+=1;
                }


                if girl.x>1 && girl.rot == 7 && map[(girl.x-1) as usize][girl.y as usize].tile == TileType::Bridge && !occupiedtiles.contains(&vec![girl.x-1, girl.y])
                {
                    girl.x-=1;
                } else if girl.x<49 && girl.rot == 3 && map[(girl.x+1) as usize][girl.y as usize].tile == TileType::Bridge && !occupiedtiles.contains(&vec![girl.x+1, girl.y])
                {
                    girl.x+=1;
                }
                if girl.y>1 && girl.rot == 6 && map[girl.x as usize][(girl.y-1) as usize].tile == TileType::Bridge && !occupiedtiles.contains(&vec![girl.x, girl.y-1])
                {
                    girl.y-=1;
                } else if girl.y<49 && girl.rot == 2 && map[girl.x as usize][(girl.y+1) as usize].tile == TileType::Bridge && !occupiedtiles.contains(&vec![girl.x, girl.y+1])
                {
                    girl.y+=1;
                }
                if girl.x<49 && girl.y>1 && girl.rot == 4 && map[(girl.x+1) as usize][(girl.y-1) as usize].tile == TileType::Bridge && !occupiedtiles.contains(&vec![girl.x+1, girl.y-1])
                {
                    girl.x+=1;
                    girl.y-=1;
                } else if girl.y<49 && girl.x>1 && girl.rot == 0 && map[(girl.x-1) as usize][(girl.y+1) as usize].tile == TileType::Bridge && !occupiedtiles.contains(&vec![girl.x-1, girl.y+1])
                {
                    girl.x-=1;
                    girl.y+=1;
                }

                if let Some(index) = occupiedtiles.iter().position(|x| x == &oldpos) {
                    occupiedtiles.remove(index);
                }
                occupiedtiles.push(vec![girl.x,girl.y]);
                
            }
            }
        }

        for girl in 0..girls.len(){
            if prevgirl[girl].mode == GirlModes::Work && girls[girl].mode == GirlModes::Idle{
                girls[girl].money += 3;
                res_monies -= 3;
            }
        }

        //girl speak 2 eachother on collision
        let gnum = girls.len();
        for girl_a in 0..gnum{
            if girls[girl_a].mode==GirlModes::Idle{
                if rand::random_bool(0.01){
                    for girl_b in 0..gnum{
                        if girl_a == girl_b {
                            continue;
                        }
                        if girls[girl_b].mode==GirlModes::Idle && girls[girl_a].cooldown==0.0 && girls[girl_b].cooldown==0.0 {
                            //girls[girl_b].rot=0
                            if  girls[girl_a].x <= girls[girl_b].x +1
                            && girls[girl_a].x >= girls[girl_b].x -1
                            && girls[girl_a].y <= girls[girl_b].y +1
                            && girls[girl_a].y >= girls[girl_b].y -1
                            {
                                girls[girl_a].mode=GirlModes::Interact;
                                girls[girl_b].mode=GirlModes::Interact;

                                if girls[girl_a].x < girls[girl_b].x && girls[girl_a].y < girls[girl_b].y{ // a left up b
                                    girls[girl_b].rot = 6;
                                    girls[girl_a].rot = 2;
                                }
                                else if girls[girl_a].x > girls[girl_b].x && girls[girl_a].y < girls[girl_b].y{ // a right up b
                                    girls[girl_b].rot = 4;
                                    girls[girl_a].rot = 0;
                                }
                                else if girls[girl_a].x < girls[girl_b].x  && girls[girl_a].y > girls[girl_b].y{ // a left down b
                                    girls[girl_a].rot = 4;
                                    girls[girl_b].rot = 0;
                                }
                                else if girls[girl_a].x > girls[girl_b].x && girls[girl_a].y > girls[girl_b].y { // a right down b
                                    girls[girl_a].rot = 6;
                                    girls[girl_b].rot = 2;
                                }
                                else if girls[girl_a].x < girls[girl_b].x { // a left b
                                    girls[girl_b].rot = 0;
                                    girls[girl_a].rot = 4;
                                }
                                else if girls[girl_a].x > girls[girl_b].x { // a right b
                                    girls[girl_b].rot = 4;
                                    girls[girl_a].rot = 0;
                                }
                                else if girls[girl_a].y < girls[girl_b].y { // a up b
                                    girls[girl_b].rot = 6;
                                    girls[girl_a].rot = 2;
                                }
                                else if girls[girl_a].y > girls[girl_b].y { // a down b
                                    girls[girl_b].rot = 2;
                                    girls[girl_a].rot = 6;
                                }
                                partners.push(vec![girl_a as i32,girl_b as i32]);
                                girls[girl_a].cooldown=10.0;
                                girls[girl_b].cooldown=10.0;

                            }
                        }
                    }
                }
            }
        }

        for bond in &partners{
            if rand::random_bool(0.01){
                if girls[bond[0] as usize].cooldown == 0.0{
                    girls[bond[0] as usize].mode=GirlModes::Idle;
                    girls[bond[1] as usize].mode=GirlModes::Idle;
                    girls[bond[0] as usize].cooldown=10.0;
                    girls[bond[1] as usize].cooldown=10.0;
                }
            }
        }
        

        for y in 0..50{
            for x in 0..50{
                d.draw_rectangle(64*x+32*y-(posx.round() as i32), 73+22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, 64, 300, bgc);
            }
        }

        for plant in &plantlocs{
            let mut canyou = true;
            let x = plant[0];
            let y = plant[1];

            for girl in &girls{
                if x == girl.x && y == girl.y{
                    canyou=false;
                }
            }
            if canyou{
                if map[x as usize][y  as usize].tile == TileType::Grass{
                    if rand::random_bool(0.0001){
                        if rand::random_bool(0.3)
                        {
                            map[x as usize][y  as usize].tile=TileType::Tree;
                        }else if rand::random_bool(0.5){
                            map[x as usize][y  as usize].tile=TileType::Bush;
                        }else {
                            map[x as usize][y  as usize].tile=TileType::Rock;
                        }
                    }
                }}
        }

        let mut hice = 0;
        for y in 0..50{
            for x in 0..50{
                if map[x as usize][y  as usize].tile==TileType::Grass{
                    d.draw_texture(&grass_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                } else if map[x as usize][y  as usize].tile==TileType::Road{
                    d.draw_texture(&road_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                } else if map[x as usize][y  as usize].tile==TileType::A{
                    if map[x as usize][y  as usize].progress<11{
                        d.draw_texture(&building_sprite[map[x as usize][y  as usize].progress as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                    } else{
                        d.draw_texture(&a_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                    }
                } else if map[x as usize][y  as usize].tile==TileType::House{
                    if map[x as usize][y  as usize].progress<11{
                        d.draw_texture(&building_sprite[map[x as usize][y  as usize].progress as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                    } else{
                        hice+=1;
                        d.draw_texture(&house_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                    }
                } else if map[x as usize][y  as usize].tile==TileType::Factory{
                    if map[x as usize][y  as usize].progress<11{
                        d.draw_texture(&building_sprite[map[x as usize][y  as usize].progress as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                    } else{
                        d.draw_texture(&factory_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                    }
                } else if map[x as usize][y  as usize].tile==TileType::Ball{
                    if map[x as usize][y  as usize].progress<11{
                        d.draw_texture(&building_sprite[map[x as usize][y  as usize].progress as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                    } else{
                        d.draw_texture(&ball_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                    }
                }  else if map[x as usize][y  as usize].tile==TileType::Wall{
                    if map[x as usize][y  as usize].progress<11{
                        d.draw_texture(&building_sprite[map[x as usize][y  as usize].progress as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                    } else{
                        d.draw_texture(&wall_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                    }
                } else if map[x as usize][y  as usize].tile==TileType::Sculpture{
                    if map[x as usize][y  as usize].progress<11{
                        d.draw_texture(&building_sprite[map[x as usize][y  as usize].progress as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                    } else{
                        d.draw_texture(&sculpture_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                    }
                }   else if map[x as usize][y  as usize].tile==TileType::Bush{
                    d.draw_texture(&bush_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                }   else if map[x as usize][y  as usize].tile==TileType::Tree{
                    d.draw_texture(&tree_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                }   else if map[x as usize][y  as usize].tile==TileType::Rock{
                    d.draw_texture(&rock_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::new(255-(map[x as usize][y  as usize].height*13) as u8, 255-(map[x as usize][y  as usize].height*13) as u8, 255, 255)); 
                } else if map[x as usize][y  as usize].tile==TileType::River{
                    map[x as usize][y  as usize].height = 0;
                    d.draw_texture(&river_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)+6, Color::WHITE); 
                }  else if map[x as usize][y  as usize].tile==TileType::Bridge{
                    map[x as usize][y  as usize].height = 0;
                    d.draw_texture(&river_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)+6, Color::WHITE);
                    d.draw_texture(&bridge_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::WHITE);  
                } 
                for girl in &girls{
                    if x==girl.x && y==girl.y{
                        if girl.col{
                            d.draw_texture(&bgirl_sprite[girl.rot as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::WHITE); 
                        }else{
                            d.draw_texture(&wgirl_sprite[girl.rot as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::WHITE); 
                        }
                        if girl.mode==GirlModes::Interact{
                            d.draw_texture(&talk_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::WHITE);
                        } else if girl.mode==GirlModes::Work{
                            d.draw_texture(&work_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::WHITE);
                        } else if girl.mode==GirlModes::Attent{
                            d.draw_texture(&alert_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::WHITE);
                        } else if girl.mode==GirlModes::Going{
                            d.draw_texture(&go_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::WHITE);
                        }
                    }
                }

            if d.get_mouse_y() > 60 && d.get_mouse_y() < 420{
                let sx = 64*x + 32*y - posx.round() as i32;
                let sy = 22*y - posy.round() as i32;

                let cx = sx + 32;
                let cy = sy + 22;

                let mx = d.get_mouse_x();
                let my = d.get_mouse_y() - 51 + map[x as usize][y  as usize].height*7;

                let dx = (mx - cx).abs();
                let dy = (my - cy).abs();

                if dx * 22 + dy * 32 <= 32 * 22 {
                    d.draw_texture(&cursor,  64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*7, Color::WHITE);

                    if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && paintbrushtype==BrushType::Tile{
                        let mut canyou = true;

                        for girl in &girls{
                            if x == girl.x && y == girl.y{
                                canyou=false;
                            }
                        }

                        if canyou{
                            if map[x as usize][y as usize].tile==TileType::Grass || map[x as usize][y as usize].tile==TileType::River || map[x as usize][y as usize].tile==TileType::Bridge{
                                let mut tmp_res = (0,0,0);

                                tmp_res = calculate_resources(paintbrush); //wood food stone
                                res_wood-=tmp_res.0;
                                res_food-=tmp_res.1;
                                res_stone-=tmp_res.2;
                                if res_wood<0 || res_food<0 || res_stone<0{
                                    let _beep = thread::spawn(|| {
                                        synth::bark();
                                    });
                                res_wood+=tmp_res.0;
                                res_food+=tmp_res.1;
                                res_stone+=tmp_res.2;
                                }else {
                                    map[x as usize][y as usize].tile = paintbrush;
                                    let _beep = thread::spawn(|| {
                                        synth::beep();
                                    });
                                    if map[x as usize][y as usize].tile == TileType::Grass || map[x as usize][y as usize].tile == TileType::Bridge || map[x as usize][y as usize].tile == TileType::River || map[x as usize][y as usize].tile == TileType::Road{
                                        map[x as usize][y as usize].progress = 12;
                                    } else {
                                        map[x as usize][y as usize].progress = 0;
                                    }
                                }
                                
                            }
                            
                        }
                    } else if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) && paintbrushtype==BrushType::Tile{
                        let mut canyou = true;

                        for girl in &girls{
                            if x == girl.x && y == girl.y{
                                canyou=false;
                            }
                        }

                        if canyou{
                            if ! (map[x as usize][y as usize].tile==TileType::Grass || map[x as usize][y as usize].tile==TileType::River || map[x as usize][y as usize].tile==TileType::Bridge){
                                let mut tmp_res = (0,0,0);
                                tmp_res = calculate_resources(map[x as usize][y as usize].tile); //wood food stone
                                res_wood+=tmp_res.0;
                                res_food+=tmp_res.1;
                                res_stone+=tmp_res.2;
                                map[x as usize][y as usize].tile = TileType::Grass;
                                let _beep = thread::spawn(|| {
                                    synth::pakala();
                                });
                            }
                            
                        }
                    } else if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && paintbrushtype==BrushType::Rise{
                        let mut canyou = true;

                        for girl in &girls{
                            if x == girl.x && y == girl.y{
                                canyou=false;
                            }
                        }

                        if canyou{
                            if map[x as usize][y as usize].height <2{
                                map[x as usize][y as usize].height += 1;
                            }
                        }
                    } else if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && paintbrushtype==BrushType::Depress{
                        let mut canyou = true;

                        for girl in &girls{
                            if x == girl.x && y == girl.y{
                                canyou=false;
                            }
                        }

                        if canyou{
                            if map[x as usize][y as usize].height >0{
                                map[x as usize][y as usize].height -= 1;
                            }
                        }
                    }else if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && paintbrushtype==BrushType::Girl{
                        let mut canyou = true;

                        for girl in &girls{
                            if x == girl.x && y == girl.y{
                                canyou=false;
                            }
                        }
                        if map[x as usize][y as usize].tile != TileType::Grass && map[x as usize][y as usize].tile != TileType::Bridge{
                            canyou=false;
                        } 

                        if canyou{
                            for girl in &mut girls{
                                if girl.mode == GirlModes::Attent{
                                    girl.mode = GirlModes::Going;
                                    girl.destination = vec![x,y];
                                    paintbrushtype = BrushType::Tile;
                                }
                            }
                        }
                    }
                    if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
                        for girl in &mut girls{
                            if x == girl.x && y == girl.y && (girl.mode==GirlModes::Idle || girl.mode==GirlModes::Going){
                                girl.mode = GirlModes::Attent;
                                paintbrushtype = BrushType::Girl;
                            }
                        }
                    }
                    
                }
            }

            }
        }
        
        
        
        d.draw_rectangle(0, 0, 768, 60, Color::LIGHTBLUE);
        d.draw_rectangle(0, 420, 768, 220, Color::LIGHTBLUE);
        //draw_text(&mut d, &font,"GirlCountry", 5, 5, 50, Color::WHITE);
        
        if paintbrushtype == BrushType::Rise{
            d.draw_rectangle(0, 470, 80, 24, Color::YELLOW);
        }
        d.draw_rectangle_lines(0, 470, 80, 24, Color::BLACK);
        draw_text(&mut d, &font,"elevate",2,472,20,Color::BLACK);

        if paintbrushtype == BrushType::Depress{
            d.draw_rectangle(0, 500, 90, 24, Color::YELLOW);
        }
        d.draw_rectangle_lines(0, 500, 90, 24, Color::BLACK);
        draw_text(&mut d, &font,"depress",2,502,20,Color::BLACK);

        if paintbrushtype != BrushType::Girl{
            if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) 
                    && d.get_mouse_x()>0
                    && d.get_mouse_x()<80
                    && d.get_mouse_y()>470
                    && d.get_mouse_y()<494
                {
                    paintbrushtype=BrushType::Rise
                }
            if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) 
                    && d.get_mouse_x()>0
                    && d.get_mouse_x()<90
                    && d.get_mouse_y()>500
                    && d.get_mouse_y()<524
                {
                    paintbrushtype=BrushType::Depress
                }
            
        }

        let tmptypearr= vec![
            TileType::Grass,
            TileType::Factory,
            TileType::A,
            TileType::House,
            TileType::Ball,
            TileType::River,
            TileType::Bridge,
            TileType::Sculpture,
            TileType::Wall,
            TileType::Road,
        ];

        for i in 0..10{
            if paintbrush == tmptypearr[i as usize] && paintbrushtype==BrushType::Tile{
                d.draw_rectangle(67*i, 540, 64, 96, Color::YELLOW);
            }
            d.draw_rectangle_lines(67*i, 540, 64, 96, Color::BLACK);
            if paintbrushtype != BrushType::Girl{
                if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) 
                    && d.get_mouse_x()>67*i
                    && d.get_mouse_x()<64+67*i
                    && d.get_mouse_y()>540
                    && d.get_mouse_y()<636
                {
                    paintbrush= tmptypearr[i as usize];
                    paintbrushtype= BrushType::Tile;
                }
            }
        }
        draw_text(&mut d, &font,"grass",2,540,17,Color::BLACK);
        d.draw_texture(&grass_sprite,0,540,Color::WHITE);
        draw_text(&mut d, &font,"factory",64+3+2,540,17,Color::BLACK);
        d.draw_texture(&factory_sprite ,64+3,540,Color::WHITE);
        draw_text(&mut d, &font," \"a\"",128+6+2,540,17,Color::BLACK);
        d.draw_texture(&a_sprite,128+6,540,Color::WHITE);
        draw_text(&mut d, &font,"house",192+9+2,540,17,Color::BLACK);
        d.draw_texture(&house_sprite ,192+9,540,Color::WHITE);
        draw_text(&mut d, &font,"ball",256+12+2,540,17,Color::BLACK);
        d.draw_texture(&ball_sprite ,256+12,540,Color::WHITE);
        draw_text(&mut d, &font,"water",320+15+2,540,17,Color::BLACK);
        d.draw_texture(&river_sprite ,320+15,540,Color::WHITE);
        draw_text(&mut d, &font,"bridge",384+18+2,540,17,Color::BLACK);
        d.draw_texture(&bridge_sprite ,384+17,540,Color::WHITE);
        draw_text(&mut d, &font,"ornment",448+21+2,540,17,Color::BLACK);
        d.draw_texture(&sculpture_sprite ,448+21,540,Color::WHITE);
        draw_text(&mut d, &font,"wall",512+24+2,540,17,Color::BLACK);
        d.draw_texture(&wall_sprite ,512+24,540,Color::WHITE);
        draw_text(&mut d, &font,"road",576+27+2,540,17,Color::BLACK);
        d.draw_texture(&road_sprite ,576+27,540,Color::WHITE);

        
        draw_text(&mut d, &font,&("food: ".to_owned()+&res_food.to_string()+"\nwood: "+&res_wood.to_string()+"\nstone: "+&res_stone.to_string()+"\nmonies: "+&res_monies.to_string()),110,450,20,Color::BLACK);
        draw_text(&mut d, &font,&("housed girls: ".to_owned()+&hice.to_string()+"/"+&girls.len().to_string()),230,450,20,Color::BLACK);

        draw_text(&mut d, &font,months[month], 602, 19, 25, Color::CYAN);
        draw_text(&mut d, &font,months[month], 600, 17, 25, Color::BLACK);

        //draw_text(&mut d, &font,"GirlCountry", 5, 5, 50, Color::BLACK);
        draw_border_text(&mut d, &font, "GirlCountry",50,5,5);


        //draw_text(&mut d, &font,&(posx.to_string()+" : "+&posy.to_string()).to_string(),5,5,20, Color::WHITE)
    }
}


fn draw_border_text(mut b:&mut RaylibDrawHandle<'_>, font: &[Font;4], txt:&str, size:i32, x:i32, y:i32){
    draw_text(&mut b, &font, &txt, x-1, y, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x+1, y, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x, y-1, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x, y+1, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x-2, y, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x+2, y, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x, y-2, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x, y+2, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x-1, y-1, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x-1, y+1, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x+1, y-1, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x+1, y+1, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x-2, y-2, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x-2, y+2, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x+2, y-2, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x+2, y+2, size, Color::BLACK);
    draw_text(&mut b, &font, &txt, x, y, size, Color::WHITE);
}


fn calculate_resources(tiletype:TileType) -> (i32,i32,i32) { //wood food stone

    if tiletype == TileType::A 
    {
        return (0,1,5);
    } else if tiletype == TileType::Ball
    {
        return (2,3,6);
    } else if tiletype == TileType::Bridge
    {
        return (1,1,0);
    } else if tiletype == TileType::Factory
    {
        return (7,5,9);
    } else if tiletype == TileType::House
    {
        return (6,2,0);
    } else if tiletype == TileType::Sculpture
    {
        return (0,9,3);
    } else if tiletype == TileType::Wall 
    {
        return (3,0,3);
    }  else if tiletype == TileType::Tree 
    {
        return (2,0,0);
    }  else if tiletype == TileType::Bush
    {
        return (0,2,0);
    }  else if tiletype == TileType::Rock 
    {
        return (0,0,2);
    } else if tiletype == TileType::Road 
    {
        return (0,1,0);
    } else
    {
        return (0,0,0);
    }


    
}


fn draw_text(b:&mut RaylibDrawHandle<'_>, fonts: &[Font;4], txt: &str, x:i32,y:i32,size:i32,color:Color){
    // 17 20 25 50
    let mut font = &fonts[0];
    if size == 17{
        font = &fonts[0];
    } else if size == 20{
        font = &fonts[1];
    } else if size == 25{
        font = &fonts[2];
    } else if size == 50{
        font = &fonts[3];
    }
    b.draw_text_ex(font, txt, raylib::prelude::Vector2::new(x as f32, y as f32), size as f32, 0.0, color);
}

//("depress",2,502,20,Color::BLACK);