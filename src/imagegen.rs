use image::Pixel;
use image::{GenericImageView, ImageBuffer, RgbaImage};

use crate::TileType;
use crate::Tile;
use crate::Girl;
use crate::GirlModes;


pub fn makeimg(map:&Vec<Vec<Tile>>, girls:&Vec<Girl>) {
    let mut img = create_image();
    for y in 0..50{
        for x in 0..50{
            if map[x as usize][y as usize].tile==TileType::River ||  map[x as usize][y as usize].tile==TileType::Bridge{
                img = embed_shape(img, x*64+32*y, (y as i32*22+73+15+6) as u32);
            } else{
                img = embed_shape(img, x*64+32*y, (y as i32*22+73+15-map[x as usize][y  as usize].height*7) as u32);
            }
            if map[x as usize][y as usize].tile==TileType::Grass{
                img = embed_image(img, "assets/grass.png", x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Tree{
                img = embed_image(img, "assets/tree.png", x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Bush{
                img = embed_image(img, "assets/bush.png", x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Road{
                img = embed_image(img, "assets/path.png", x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Rock{
                img = embed_image(img, "assets/stone.png", x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Ball{
                img = embed_image(img, "assets/ball.png", x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::A{
                img = embed_image(img, "assets/a.png", x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Factory{
                img = embed_image(img, "assets/factory.png", x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::House{
                img = embed_image(img, "assets/house.png", x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::River{
                img = embed_image(img, "assets/water.png", x*64+32*y, (y as i32*22+15+6) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Bridge{
                img = embed_image(img, "assets/water.png", x*64+32*y, (y as i32*22+15+6) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
                img = embed_image(img, "assets/bridge.png", x*64+32*y, (y as i32*22+15) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            } else if map[x as usize][y as usize].tile==TileType::Wall{
                img = embed_image(img, "assets/wall.png", x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
            }  else if map[x as usize][y as usize].tile==TileType::Sculpture{
                img = embed_image(img, "assets/sculpture.png", x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [(255-(map[x as usize][y  as usize].height*13)) as u8, (255-(map[x as usize][y  as usize].height*13)) as u8, 255]);
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
                        img = embed_image(img, &("assets/charab".to_owned()+&(girl.rot+1).to_string()+".png"), x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [255, 255, 255]);
                    }else{
                        img = embed_image(img, &("assets/charaw".to_owned()+&(girl.rot+1).to_string()+".png"), x*64+32*y, (y as i32*22+15-map[x as usize][y  as usize].height*7) as u32, [255, 255, 255]);
                    }
                }
            }
        }
    }
    

    img.save("map.png").unwrap();

}

fn embed_image(mut img: RgbaImage, infile: &str, start_x: u32, start_y: u32, shade:[u8;3]) -> RgbaImage {
    let logo = image::open(infile).unwrap();

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
        for y in 0..4704 {
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