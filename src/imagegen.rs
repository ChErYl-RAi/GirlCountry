use image::{DynamicImage, Pixel};
use image::{GenericImageView, ImageBuffer, RgbaImage};

use crate::TileType;
use crate::Tile;
use crate::Girl;


pub fn makeimg(map:&Vec<Vec<Tile>>, girls:&Vec<Girl>) {
    let mut img = create_image();


    
    let bgirl_sprite= vec![
        image::open("assets/charab1.png").unwrap(),
        image::open("assets/charab2.png").unwrap(),
        image::open("assets/charab3.png").unwrap(),
        image::open("assets/charab4.png").unwrap(),
        image::open("assets/charab5.png").unwrap(),
        image::open("assets/charab6.png").unwrap(),
        image::open("assets/charab7.png").unwrap(),
        image::open("assets/charab8.png").unwrap(),
    ];

    let wgirl_sprite= vec![
        image::open("assets/charaw1.png").unwrap(),
        image::open("assets/charaw2.png").unwrap(),
        image::open("assets/charaw3.png").unwrap(),
        image::open("assets/charaw4.png").unwrap(),
        image::open("assets/charaw5.png").unwrap(),
        image::open("assets/charaw6.png").unwrap(),
        image::open("assets/charaw7.png").unwrap(),
        image::open("assets/charaw8.png").unwrap(),
    ];

    let grass = image::open("assets/grass.png").unwrap();
    let tree = image::open("assets/tree.png").unwrap();
    let bush = image::open("assets/bush.png").unwrap();
    let path = image::open("assets/path.png").unwrap();
    let stone = image::open("assets/stone.png").unwrap();
    let ball = image::open("assets/ball.png").unwrap();
    let a = image::open("assets/a.png").unwrap();
    let factory = image::open("assets/factory.png").unwrap();
    let house = image::open("assets/house.png").unwrap();
    let water = image::open("assets/water.png").unwrap();
    let bridge = image::open("assets/bridge.png").unwrap();
    let sculpture = image::open("assets/sculpture.png").unwrap();
    let wall = image::open("assets/wall.png").unwrap();




    for y in 0..50{
        for x in 0..50{
            if map[x as usize][y as usize].tile==TileType::River ||  map[x as usize][y as usize].tile==TileType::Bridge{
                img = embed_shape(img, x*64+32*y, (y as i32*22+73+15+6) as u32);
            } else{
                img = embed_shape(img, x*64+32*y, (y as i32*22+73+15-map[x as usize][y  as usize].height*7) as u32);
            }
            if map[x as usize][y as usize].tile==TileType::Grass{
                img = embed_image(img, &grass, x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Tree{
                img = embed_image(img, &tree, x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Bush{
                img = embed_image(img, &bush, x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Road{
                img = embed_image(img, &path, x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Rock{
                img = embed_image(img, &stone, x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Ball{
                img = embed_image(img, &ball, x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::A{
                img = embed_image(img, &a, x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Factory{
                img = embed_image(img, &factory, x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::House{
                img = embed_image(img, &house, x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::River{
                img = embed_image(img, &water, x*64+32*y, (y as i32*22+15+6) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Bridge{
                img = embed_image(img, &water, x*64+32*y, (y as i32*22+15+6) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
                img = embed_image(img, &bridge, x*64+32*y, (y as i32*22+15) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Wall{
                img = embed_image(img, &wall, x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            }  else if map[x as usize][y as usize].tile==TileType::Sculpture{
                img = embed_image(img, &sculpture, x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } 


            /*
            Grass, !!!!!!!!!!!!!!!
            Ball, !!!!!!!!!!!!!!!
            A, !!!!!!!!!!!!!!!
            Factory, !!!!!!!!!!!!!!!
            House, !!!!!!!!!!!!!!!
            River, !!!!!!!!!!!!!!!
            Bridge, !!!!!!!!!!!!!!!
            Tree, !!!!!!!!!!!!!!!
            Rock, !!!!!!!!!!!!!!!
            Bush, !!!!!!!!!!!!!!!
            Wall,
            Sculpture,
            Road, !!!!!!!!!!!!!!!
            
            */
            for girl in girls{
                if girl.x == x as i32 && girl.y == y as i32{
                    if girl.col{
                        img = embed_image(img, &bgirl_sprite[girl.rot as usize], x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [255, 255, 255]);
                    }else{
                        img = embed_image(img, &wgirl_sprite[girl.rot as usize], x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [255, 255, 255]);
                    }
                }
            }
        }
    }
    

    img.save("map.png").unwrap();

}

fn embed_image(mut img: RgbaImage, logo: &DynamicImage, start_x: u32, start_y: u32, shade:[u8;3]) -> RgbaImage {
    //let logo = image::open(infile).unwrap();

    //println!("Embedding:  width={}, height={}", logo.width(), logo.height());
    //println!("Base image: width={}, height={}", img.width(), img.height());

    if start_x + logo.width() > img.width() {
        //println!("Does not fit in width");
        return img;
    }
    if start_y + logo.height() > img.height() {
        //println!("Does not fit in height");
        return img;
    }

    for x in 0..logo.width() {
        for y in 0..logo.height() {
            if logo.get_pixel(x, y).to_rgba().alpha()>0{
                let fr = ((logo.get_pixel(x, y)[0] as u32 *shade[0] as u32)/255) as u8;
                let fg = ((logo.get_pixel(x, y)[1] as u32 *shade[1] as u32)/255) as u8;
                let fb = ((logo.get_pixel(x, y)[2] as u32 *shade[2] as u32)/255) as u8;
                *img.get_pixel_mut(start_x + x, start_y + y) = image::Rgba([fr,fg,fb,255]);
            }
        }
    }

    img
}

fn embed_shape(mut img: RgbaImage, start_x: u32, start_y: u32) -> RgbaImage {

    for x in 0..64 {
        for y in 0..1078 {
            if start_x+x<4704 && start_y+y<1078{
                *img.get_pixel_mut(start_x + x, start_y + y) = image::Rgba([1, 24, 0,255]);
            }
        }
    }

    img
}

fn create_image() -> RgbaImage {
    let width = 4704;
    let height = 1078;

    let mut img: RgbaImage = ImageBuffer::new(width, height);
    let red = 161 as u8;
    let green = 175;
    let blue = 212;
    let alpha = 255;

    for x in 0..width {
        for y in 0..height {
            *img.get_pixel_mut(x, y) = image::Rgba([red, green, blue, alpha]);
        }
    }

    img
    
}