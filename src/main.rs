use image::io::Reader as ImageReader;
use image::ImageError;
use image::Luma;
use argh::FromArgs;
use image::ImageBuffer;
use image::{Rgb, RgbImage};
use image::Pixel;

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {

    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    mode: Mode
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {}


#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
struct OptsPalette {

    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize
}
 
const WHITE: image::Rgb<u8> = image::Rgb([255, 255, 255]);
const GREY: image::Rgb<u8> = image::Rgb([127, 127, 127]);
const BLACK: image::Rgb<u8> = image::Rgb([0, 0, 0]);
const BLUE: image::Rgb<u8> = image::Rgb([0, 0, 255]);
const RED: image::Rgb<u8> = image::Rgb([255, 0, 0]);
const GREEN: image::Rgb<u8> = image::Rgb([0, 255, 0]);
const YELLOW: image::Rgb<u8> = image::Rgb([255, 255, 0]);
const MAGENTA: image::Rgb<u8> = image::Rgb([255, 0, 255]);
const CYAN: image::Rgb<u8> = image::Rgb([0, 255, 255]);


fn pixel_luminosity(img:RgbImage, x: u32, y: u32) -> u8 {
    let Luma(luminosite_) = img.get_pixel(x,y).to_luma();
    return luminosite_[0];
}

fn main() -> Result<(), ImageError>{

    //let args: DitherArgs = argh::from_env();
    //let path_in = args.input;
    

    //1
    /* let img_file = ImageReader::open("image.jpg")?;
    let mut img = img_file.decode()?.into_rgb8();
    println!("Le pixel en 32, 52 a pour couleur {:?}", img.get_pixel(32, 52));
    for (x, y, color) in img.enumerate_pixels_mut(){
        if (x+y) % 2  == 0{
            *color = Rgb([255,255,255])
        }
    }
    img.save("out1.png")?;
 */
    //2
    let img_file = ImageReader::open("image.jpg")?;
    let mut img = img_file.decode()?.into_rgb8();
    //let grayscale = img.to_luma();
    

    for (x, y, pixel) in img.enumerate_pixels_mut() {

        let Luma(luminosite_) = pixel.to_luma();
        
        let new_pixel = if luminosite_[0] > 127 { Rgb([255,255,255]) } else { Rgb([0,0,0]) };
        
        *pixel = new_pixel;
    }

    img.save("out2.png")?; 
    return Ok(()) 
   


}

