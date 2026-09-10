use raylib::{ffi::{CSSPalette, RaylibPalette}, prelude::*};
use rand;

#[derive(PartialEq)]
enum TileType {
    Grass,
    Ball,
    A,
    Factory,
    House,
    River,
}

struct Tile{
    tile:TileType,
    height:i32,
}

struct Girl{
    x:i32,
    y:i32,
    rot:i32,
    col:bool,
}

fn main(){
    let (mut rl, thread) = raylib::init()
        .size(768, 640)
        .title("GirlCountry")
        .build();
    let grass_sprite = rl.load_texture(&thread, "assets/grass.png").unwrap();
    let a_sprite = rl.load_texture(&thread, "assets/a.png").unwrap();
    let factory_sprite = rl.load_texture(&thread, "assets/factory.png").unwrap();
    let house_sprite = rl.load_texture(&thread, "assets/house.png").unwrap();
    let ball_sprite = rl.load_texture(&thread, "assets/ball.png").unwrap();
    let river_sprite = rl.load_texture(&thread, "assets/water.png").unwrap();

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



    let mut posx: f32 = 0.0;
    let mut posy: f32 = 0.0;
    let mut map: Vec<Vec<Tile>> = vec![];
    for i in 0..50{
        let mut tmp: Vec<Tile> = vec![];
        for j in 0..50{
            if i <23 || i>27{
                if j > 3{
                    if rand::random_bool(0.1){
                        tmp.push(
                            Tile{
                                tile: TileType::House,
                                height: 0
                            }
                        );
                    } else {
                        tmp.push(
                            Tile{
                                tile: TileType::Grass,
                                height: 0
                            }
                        );
                        if rand::random_bool(0.08){
                            girls.push(
                                Girl { x: i, y: j, rot: rand::random_range(0..8), col: rand::random_bool(0.5) }
                            );
                        }
                    }
                } else {
                    if rand::random_bool(0.1){
                        tmp.push(
                            Tile{
                                tile: TileType::House,
                                height: 1
                            }
                        );
                    } else {
                        tmp.push(
                            Tile{
                                tile: TileType::Grass,
                                height: 1
                            }
                        );
                    }
                }
            } else {
                tmp.push(
                    Tile{
                        tile: TileType::River,
                        height: 0
                    }
                );
            }
        }
        map.push(tmp);
    }
    let bgc = Color::from_hex("011800").unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(bgc);
        

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

        for girl in &mut girls{
            if rand::random_bool(0.001){
                girl.rot += 1;
                if girl.rot ==-1{
                    girl.rot=7;
                } else if girl.rot ==8 {
                    girl.rot=0;
                }
            }else if rand::random_bool(0.001) {
                girl.rot -= 1;
                if girl.rot ==-1{
                    girl.rot=7;
                } else if girl.rot ==8 {
                    girl.rot=0;
                }
            }

            if rand::random_bool(0.001){
                if girl.x>1 && girl.rot == 8 && map[(girl.x-1) as usize][girl.y as usize].tile == TileType::Grass
                {
                    girl.x-=1;
                } else if girl.x<49 && girl.rot == 3 && map[(girl.x+1) as usize][girl.y as usize].tile == TileType::Grass
                {
                    girl.x+=1;
                }
                if girl.y>1 && girl.rot == 6 && map[girl.x as usize][(girl.y-1) as usize].tile == TileType::Grass
                {
                    girl.y-=1;
                } else if girl.y<49 && girl.rot == 2 && map[girl.x as usize][(girl.y+1) as usize].tile == TileType::Grass
                {
                    girl.y+=1;
                }
            }
        }

        for y in 0..50{
            for x in 0..50{
                if map[x as usize][y  as usize].tile==TileType::Grass{
                    d.draw_texture(&grass_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                } else if map[x as usize][y  as usize].tile==TileType::A{
                    d.draw_texture(&a_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                } else if map[x as usize][y  as usize].tile==TileType::House{
                    d.draw_texture(&house_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                } else if map[x as usize][y  as usize].tile==TileType::River{
                    d.draw_texture(&river_sprite, 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)+6, Color::WHITE); 
                } 
                for girl in &girls{
                    if x==girl.x && y==girl.y{
                        let gx = girl.x;
                        let gy = girl.y;
                        if girl.col{
                            d.draw_texture(&bgirl_sprite[girl.rot as usize], 64*x+32*y-(posx.round() as i32), 22*y-(posy.round() as i32)-map[x as usize][y  as usize].height*12, Color::WHITE); 
                        }
                    }
                }
            }
        }

        
        
        
        d.draw_rectangle(0, 0, 768, 60, Color::LIGHTBLUE);
        d.draw_rectangle(0, 420, 768, 220, Color::LIGHTBLUE);
        //d.draw_text("GirlCountry", 5, 5, 50, Color::WHITE);
        
        d.draw_rectangle_lines(0, 470, 80, 24, Color::BLACK);
        d.draw_text("elevate",2,472,20,Color::BLACK);
        d.draw_rectangle_lines(0, 500, 90, 24, Color::BLACK);
        d.draw_text("depress",2,502,20,Color::BLACK);

        for i in 0..5{
            d.draw_rectangle_lines(67*i, 540, 64, 96, Color::BLACK);
        }
        d.draw_text("grass",0,540,17,Color::BLACK);
        d.draw_texture(&grass_sprite,0,540,Color::WHITE);
        d.draw_text("factory",64+3,540,17,Color::BLACK);
        d.draw_texture(&factory_sprite ,64+3,540,Color::WHITE);
        d.draw_text(" \"a\"",128+6,540,17,Color::BLACK);
        d.draw_texture(&a_sprite,128+6,540,Color::WHITE);
        d.draw_text("house",192+9,540,17,Color::BLACK);
        d.draw_texture(&house_sprite ,192+9,540,Color::WHITE);
        d.draw_text("ball",256+12,540,17,Color::BLACK);
        d.draw_texture(&ball_sprite ,256+12,540,Color::WHITE);


        draw_border_text(d, "GirlCountry",50,5,5);


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