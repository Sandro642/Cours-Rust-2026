use ma_crate::CouleurPrimaire;
use ma_crate::mixer;

fn main() {
    let rouge = CouleurPrimaire::Rouge;
    let jaune = CouleurPrimaire::Jaune;

    mixer(rouge, jaune);
}
