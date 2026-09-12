use raylib::{ffi::{CSSPalette, RaylibPalette}, prelude::*};
use rand;
use std::{i32, thread};

mod synth;

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
    Bush,
    Wall,
    Sculpture,
}


#[derive(PartialEq)]
enum BrushType {
    Tile,
    Rise,
    Depress,
}

struct Tile{
    tile:TileType,
    height:i32,
    progress:i32,
}

#[derive(PartialEq)]
enum GirlModes{
    Idle,
    Going,
    Interact,
    Work,
}

struct Girl{
    x:i32,
    y:i32,
    rot:i32,
    col:bool,
    mode: GirlModes,
    cooldown:f32,
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

    let res_wood =  10;
    let res_food =  10;
    let res_stone = 10;
    
    let mut posx: f32 = rand::random_range(400..3600) as f32;
    let mut posy: f32 = rand::random_range(-30..770) as f32;
    let mut map: Vec<Vec<Tile>> = vec![];
    let mut occupiedtiles: Vec<Vec<i32>> = vec![];
    let mut paintbrush = TileType::A;
    let mut paintbrushtype = BrushType::Tile;
    let mut partners: Vec<Vec<i32>> = vec![];
    let mut plantlocs: Vec<Vec<i32>> = vec![];

    for i in 0..50{
        let mut tmp: Vec<Tile> = vec![];
        for j in 0..50{
            if i <23 || i>27{
                if j > 3{
                    if rand::random_bool(0.1){
                        if rand::random_bool(0.5){
                            tmp.push(
                                Tile{
                                    tile: TileType::Tree,
                                    height: 0,
                                    progress:12
                                }
                            );
                            plantlocs.push(vec![i,j]);
                        }else{
                            tmp.push(
                                Tile{
                                    tile: TileType::Bush,
                                    height: 0,
                                    progress:12
                                }
                            );
                            plantlocs.push(vec![i,j]);
                        }
                    } else {
                        tmp.push(
                            Tile{
                                tile: TileType::Grass,
                                height: 0,
                                progress:12
                            }
                        );
                        if rand::random_bool(0.04){
                            girls.push(
                                Girl { x: i, y: j, rot: rand::random_range(0..8), col: rand::random_bool(0.5), mode:GirlModes::Idle, cooldown:1.0,}
                            );
                            occupiedtiles.push(vec![i,j]);
                        }
                    }
                } else {
                    if rand::random_bool(0.1){
                        tmp.push(
                            Tile{
                                tile: TileType::Bush,
                                height: 1,
                                progress:12
                            }
                        );
                        plantlocs.push(vec![i,j]);
                    } else {
                        tmp.push(
                            Tile{
                                tile: TileType::Grass,
                                height: 1,
                                progress:12
                            }
                        );
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
            month+=1;
            if month ==13 { month =0; } ;
            music.join().unwrap();
            let mus2 =months.clone();
            let mxs2 = month.clone();
            music = thread::spawn(move || {
                synth::play(mus2[mxs2]);
            });
        }
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
                d.draw_rectangle(64*x+32*y-(posx.round() as i32), 73+22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, 64, 300, bgc);
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
                    if rand::random_bool(0.00001){
                        if rand::random_bool(0.5)
                        {
                            map[x as usize][y  as usize].tile=TileType::Tree;
                        }else{
                            map[x as usize][y  as usize].tile=TileType::Bush;
                        }
                    }
                }}
        }

        for y in 0..50{
            for x in 0..50{
                if map[x as usize][y  as usize].tile==TileType::Grass{
                    d.draw_texture(&grass_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                } else if map[x as usize][y  as usize].tile==TileType::A{
                    if map[x as usize][y  as usize].progress<11{
                        d.draw_texture(&building_sprite[map[x as usize][y  as usize].progress as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                    } else{
                        d.draw_texture(&a_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                    }
                } else if map[x as usize][y  as usize].tile==TileType::House{
                    if map[x as usize][y  as usize].progress<11{
                        d.draw_texture(&building_sprite[map[x as usize][y  as usize].progress as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                    } else{
                        d.draw_texture(&house_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                    }
                } else if map[x as usize][y  as usize].tile==TileType::Factory{
                    if map[x as usize][y  as usize].progress<11{
                        d.draw_texture(&building_sprite[map[x as usize][y  as usize].progress as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                    } else{
                        d.draw_texture(&factory_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                    }
                } else if map[x as usize][y  as usize].tile==TileType::Ball{
                    if map[x as usize][y  as usize].progress<11{
                        d.draw_texture(&building_sprite[map[x as usize][y  as usize].progress as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                    } else{
                        d.draw_texture(&ball_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                    }
                }  else if map[x as usize][y  as usize].tile==TileType::Wall{
                    if map[x as usize][y  as usize].progress<11{
                        d.draw_texture(&building_sprite[map[x as usize][y  as usize].progress as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                    } else{
                        d.draw_texture(&wall_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                    }
                } else if map[x as usize][y  as usize].tile==TileType::Sculpture{
                    if map[x as usize][y  as usize].progress<11{
                        d.draw_texture(&building_sprite[map[x as usize][y  as usize].progress as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                    } else{
                        d.draw_texture(&sculpture_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                    }
                }   else if map[x as usize][y  as usize].tile==TileType::Bush{
                    d.draw_texture(&bush_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                }   else if map[x as usize][y  as usize].tile==TileType::Tree{
                    d.draw_texture(&tree_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                } else if map[x as usize][y  as usize].tile==TileType::River{
                    map[x as usize][y  as usize].height = 0;
                    d.draw_texture(&river_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)+6, Color::WHITE); 
                }  else if map[x as usize][y  as usize].tile==TileType::Bridge{
                    map[x as usize][y  as usize].height = 0;
                    d.draw_texture(&river_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)+6, Color::WHITE);
                    d.draw_texture(&bridge_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE);  
                } 
                for girl in &girls{
                    if x==girl.x && y==girl.y{
                        if girl.col{
                            d.draw_texture(&bgirl_sprite[girl.rot as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                        }else{
                            d.draw_texture(&wgirl_sprite[girl.rot as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                        }
                        if girl.mode==GirlModes::Interact{
                            d.draw_texture(&talk_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE);
                        } else if girl.mode==GirlModes::Work{
                            d.draw_texture(&work_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE);
                        }
                    }
                }

            if d.get_mouse_y() > 60 && d.get_mouse_y() < 420{
                let sx = 64*x + 32*y - posx.round() as i32;
                let sy = 22*y - posy.round() as i32;

                let cx = sx + 32;
                let cy = sy + 22;

                let mx = d.get_mouse_x();
                let my = d.get_mouse_y() - 51;

                let dx = (mx - cx).abs();
                let dy = (my - cy).abs();

                if dx * 22 + dy * 32 <= 32 * 22 {
                    d.draw_texture(&cursor,  64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE);

                    if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && paintbrushtype==BrushType::Tile{
                        let mut canyou = true;

                        for girl in &girls{
                            if x == girl.x && y == girl.y{
                                canyou=false;
                            }
                        }

                        if canyou{
                            if map[x as usize][y as usize].tile==TileType::Grass || map[x as usize][y as usize].tile==TileType::River || map[x as usize][y as usize].tile==TileType::Bridge{
                                map[x as usize][y as usize].tile = paintbrush;
                                let _beep = thread::spawn(|| {
                                    synth::beep();
                                });
                                if map[x as usize][y as usize].tile == TileType::Grass || map[x as usize][y as usize].tile == TileType::Bridge || map[x as usize][y as usize].tile == TileType::River{
                                    map[x as usize][y as usize].progress = 12;
                                } else {
                                    map[x as usize][y as usize].progress = 0;
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
                                map[x as usize][y as usize].tile = TileType::Grass;
                                let _beep = thread::spawn(|| {
                                    synth::beep();
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
                    }
                }
            }

            }
        }
        
        
        
        d.draw_rectangle(0, 0, 768, 60, Color::LIGHTBLUE);
        d.draw_rectangle(0, 420, 768, 220, Color::LIGHTBLUE);
        //d.draw_text("GirlCountry", 5, 5, 50, Color::WHITE);
        
        if paintbrushtype == BrushType::Rise{
            d.draw_rectangle(0, 470, 80, 24, Color::YELLOW);
        }
        d.draw_rectangle_lines(0, 470, 80, 24, Color::BLACK);
        d.draw_text("elevate",2,472,20,Color::BLACK);

        if paintbrushtype == BrushType::Depress{
            d.draw_rectangle(0, 500, 90, 24, Color::YELLOW);
        }
        d.draw_rectangle_lines(0, 500, 90, 24, Color::BLACK);
        d.draw_text("depress",2,502,20,Color::BLACK);


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
        ];

        for i in 0..9{
            if paintbrush == tmptypearr[i as usize] && paintbrushtype==BrushType::Tile{
                d.draw_rectangle(67*i, 540, 64, 96, Color::YELLOW);
            }
            d.draw_rectangle_lines(67*i, 540, 64, 96, Color::BLACK);

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
        d.draw_text("grass",2,540,17,Color::BLACK);
        d.draw_texture(&grass_sprite,0,540,Color::WHITE);
        d.draw_text("factory",64+3+2,540,17,Color::BLACK);
        d.draw_texture(&factory_sprite ,64+3,540,Color::WHITE);
        d.draw_text(" \"a\"",128+6+2,540,17,Color::BLACK);
        d.draw_texture(&a_sprite,128+6,540,Color::WHITE);
        d.draw_text("house",192+9+2,540,17,Color::BLACK);
        d.draw_texture(&house_sprite ,192+9,540,Color::WHITE);
        d.draw_text("ball",256+12+2,540,17,Color::BLACK);
        d.draw_texture(&ball_sprite ,256+12,540,Color::WHITE);
        d.draw_text("water",320+15+2,540,17,Color::BLACK);
        d.draw_texture(&river_sprite ,320+15,540,Color::WHITE);
        d.draw_text("bridge",384+18+2,540,17,Color::BLACK);
        d.draw_texture(&bridge_sprite ,384+17,540,Color::WHITE);
        d.draw_text("ornment",448+21+2,540,17,Color::BLACK);
        d.draw_texture(&sculpture_sprite ,448+21,540,Color::WHITE);
        d.draw_text("wall",512+24+2,540,17,Color::BLACK);
        d.draw_texture(&wall_sprite ,512+24,540,Color::WHITE);


        d.draw_text(&("wood: ".to_string()+&res_wood.to_string()+"\nfood: "+&res_food.to_string()+"\nstone: "+&res_stone.to_string()),110,460,20,Color::BLACK);

        d.draw_text(months[month], 602, 19, 25, Color::CYAN);
        d.draw_text(months[month], 600, 17, 25, Color::BLACK);

        d.draw_text("GirlCountry", 5, 5, 50, Color::BLACK);
        //draw_border_text(d, "GirlCountry",50,5,5);


        //d.draw_text(&(posx.to_string()+" : "+&posy.to_string()).to_string(),5,5,20, Color::WHITE)
    }
}


fn draw_border_text(mut d:RaylibDrawHandle<'_>, txt:&str, size:i32, x:i32, y:i32){
    d.draw_text(&txt, x-1, y, size, Color::BLACK);
    d.draw_text(&txt, x+1, y, size, Color::BLACK);
    d.draw_text(&txt, x, y-1, size, Color::BLACK);
    d.draw_text(&txt, x, y+1, size, Color::BLACK);
    d.draw_text(&txt, x-2, y, size, Color::BLACK);
    d.draw_text(&txt, x+2, y, size, Color::BLACK);
    d.draw_text(&txt, x, y-2, size, Color::BLACK);
    d.draw_text(&txt, x, y+2, size, Color::BLACK);

    d.draw_text(&txt, x-1, y-1, size, Color::BLACK);
    d.draw_text(&txt, x-1, y+1, size, Color::BLACK);
    d.draw_text(&txt, x+1, y-1, size, Color::BLACK);
    d.draw_text(&txt, x+1, y+1, size, Color::BLACK);

    d.draw_text(&txt, x-2, y-2, size, Color::BLACK);
    d.draw_text(&txt, x-2, y+2, size, Color::BLACK);
    d.draw_text(&txt, x+2, y-2, size, Color::BLACK);
    d.draw_text(&txt, x+2, y+2, size, Color::BLACK);

    d.draw_text(&txt, x, y, size, Color::WHITE);
}