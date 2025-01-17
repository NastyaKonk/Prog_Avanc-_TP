use image::io::Reader as ImageReader;
use image::ImageError;
use image::Luma;
use argh::FromArgs;
use image::ImageBuffer;
use image::{Rgb, RgbImage};
use image::Pixel;
use rand::Rng;

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
    Dithering(OptsDithering),
    Brouillard(OptsBrouillard)
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {

    /// couleur pour les pixels sombres (choisir parmi: noir, blanc, rouge, vert, bleu, jaune, cyan, magenta)
    #[argh(option, short = 'd', default = "default_noir()")]
    dark_color: String,

    /// couleur pour les pixels clairs (choisir parmi: noir, blanc, rouge, vert, bleu, jaune, cyan, magenta)
    #[argh(option, short = 'l', default = "default_blanc()")]
    light_color: String,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="Dithering")]
///Rendu du dithering sur l'image
struct OptsDithering {
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="Brouillard")]
///Rendu de l'mage quand on passe un pixel sur deux en blanc
struct OptsBrouillard {


   
}
#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
struct OptsPalette {

    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option, default = "default_number()")]
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

fn default_number() -> usize {
    8
}

fn default_noir() -> String {
    "noir".to_string()
}

fn default_blanc() -> String {
    "blanc".to_string()
}

fn parse_color(name: &str) -> Rgb<u8> {
    match name {
        "noir" => BLACK,
        "blanc" => WHITE,
        "rouge" => RED,
        "vert" => GREEN,
        "bleu" => BLUE,
        "jaune" => YELLOW,
        "cyan" => CYAN,
        "magenta" => MAGENTA,
        _ => panic!("Couleur invalide: {}", name),
    }
}

fn find_closest_color(pixel: &Rgb<u8>, palette: &[Rgb<u8>]) -> Rgb<u8> {
    let mut closest_color = palette[0];
    let mut smallest_distance = f64::MAX;

    for &color in palette {
        let distance = color_distance(pixel, &color);
        if distance < smallest_distance {
            smallest_distance = distance;
            closest_color = color;
        }
    }

    closest_color
}

fn color_distance(c1: &Rgb<u8>, c2: &Rgb<u8>) -> f64 {
    let dr = c1[0] as f64 - c2[0] as f64;
    let dg = c1[1] as f64 - c2[1] as f64;
    let db = c1[2] as f64 - c2[2] as f64;
    (dr * dr + dg * dg + db * db).sqrt()
}

fn main() -> Result<(), ImageError>{

    let args: DitherArgs = argh::from_env();
    let path_in = args.input;
    let path_out = args.output.unwrap_or_else(|| "output.png".to_string());
    

    
    let img_file = ImageReader::open(path_in)?;
    let mut img = img_file.decode()?.into_rgb8();
    println!("Le pixel en 32, 52 a pour couleur {:?}", img.get_pixel(32, 52));
  
   
    match args.mode {
        Mode::Seuil(opts) => {
            for (_x, _y, pixel) in img.enumerate_pixels_mut() {
                let dark_color = parse_color(&opts.dark_color);
                let light_color = parse_color(&opts.light_color);

                let Luma(luminosity) = pixel.to_luma();
                let new_pixel = if luminosity[0] > 127 {
                    light_color
                } else {
                    dark_color
                };
                *pixel = new_pixel;
            }
        }
        Mode::Palette(opts) => {
            let palette = match opts.n_couleurs {
                1 => vec![BLACK],
                2 => vec![BLACK, WHITE],
                3 => vec![BLACK, WHITE, RED],
                4 => vec![BLACK, WHITE, RED, GREEN],
                5 => vec![BLACK, WHITE, RED, GREEN, BLUE],
                6 => vec![BLACK, WHITE, RED, GREEN, BLUE, YELLOW],
                7 => vec![BLACK, WHITE, RED, GREEN, BLUE, YELLOW, CYAN],
                8 => vec![BLACK, WHITE, RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA],
                _ => panic!("Nombre de couleurs invalide: {}", opts.n_couleurs),
            };

            for (_x, _y, pixel) in img.enumerate_pixels_mut() {
                *pixel = find_closest_color(pixel, &palette);
            }
        }

        Mode::Dithering(opts) => {
    

            for (_x, _y, pixel) in img.enumerate_pixels_mut() {
                // Calculer la luminosité du pixel
                let Luma(luminosity) = pixel.to_luma();
            
                // Générer un seuil aléatoire entre 0 et 255
                let random_threshold: f32 = rand::thread_rng().gen_range(0.0..1.0);
            
                // Comparer la luminosité normalisée au seuil
                let new_pixel = if luminosity[0] as f32 / 255.0 > random_threshold {
                    WHITE // Si luminosité supérieure au seuil, pixel blanc
                } else {
                    BLACK // Sinon, pixel noir
                };
            
                *pixel = new_pixel;
            }
        }

        Mode::Brouillard(opts) => {
            for (x, y, color) in img.enumerate_pixels_mut(){
                if (x+y) % 2  == 0{
                    *color = Rgb([255,255,255])
                }
            }
    
        }
    }

    img.save(path_out)?;
    return Ok(()) 
   


}

