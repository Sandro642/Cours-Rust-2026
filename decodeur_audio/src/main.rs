fn main() {
    let tampon: &mut [i32];
    let coefficients: [i64; 12];
    let decalage: i16;

    for i in 12..tampon.len() {
        let prediction = coefficients
            .iter()
            .zip(&tampon[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>()
            >> decalage;

        let delta = tampon[i];
        tampon[i] = prediction as i32 + delta;
    }
}
