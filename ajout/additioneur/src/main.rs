use ajouter_deux;
use ajouter_un;

fn main() {
    let nombre = 10;

    println!(
        "Hello, world ! {} plus un vaut {} !",
        nombre,
        ajouter_un::ajouter_un(nombre)
    );

    println!(
        "Hello, world ! {} plus deux vaut {} !",
        nombre,
        ajouter_deux::ajouter_deux(nombre)
    );
}
