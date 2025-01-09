use argh::FromArgs;
use image::{open, RgbImage};

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {
    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    /// sous-commandes disponibles
    #[argh(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Command {
    Luminosite(LuminositeArgs),  // La sous-commande Luminosite
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Arguments pour afficher la luminosité d'un pixel
struct LuminositeArgs {
    /// coordonnées du pixel
    #[argh(positional)]
    x: u32,
    #[argh(positional)]
    y: u32,
}

// Fonction pour calculer la luminosité d'un pixel RGB
fn calculate_luminance(r: u8, g: u8, b: u8) -> f32 {
    0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32
}

fn main() {
    // Extraire les arguments en utilisant argh
    let args: DitherArgs = argh::from_env();

    // Ouvrir l'image spécifiée dans les arguments
    let img = open(&args.input)
        .expect("Échec de l'ouverture de l'image")
        .to_rgb8();

    // Gestion de la sous-commande Luminosite
    match args.command {
        Command::Luminosite(lum_args) => {
            // Vérifier si les coordonnées sont dans les limites de l'image
            if lum_args.x < img.width() && lum_args.y < img.height() {
                let pixel = img.get_pixel(lum_args.x, lum_args.y);
                let luminance = calculate_luminance(pixel[0], pixel[1], pixel[2]);
                println!(
                    "La luminosité du pixel ({}, {}) est : {:.2}",
                    lum_args.x, lum_args.y, luminance
                );
            } else {
                println!("Les coordonnées ({}, {}) sont hors de l'image.", lum_args.x, lum_args.y);
            }
        }
    }
}