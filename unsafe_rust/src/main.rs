use std::slice;

fn main() {
    let mut nombre = 5;

    let r1 = &nombre as *const i32;
    let r2 = &mut nombre as *mut i32;

    let adresse = 0x012345usize;
    let r = adresse as *const i32;

    println!("{:?}", r);

    unsafe {
        println!("r1 vaut : {}", *r1);
        println!("r2 vaut : {}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);

    let r = adresse as *mut i32;
    //let valeurs: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("La valeur absolue de -3 selong le langage C : {}", abs(-3));
    }

    println!("Cela vaut : {}", HELLO_WORLD);

    ajouter_au_compteur(3);

    // unsafe {
    //     println!("COMPTEUR : {}", COMPTEUR);
    // }
}

unsafe trait Foo {}

unsafe impl Foo for i32 {}

static mut COMPTEUR: u32 = 0;

fn ajouter_au_compteur(valeur: u32) {
    unsafe {
        COMPTEUR += valeur;
    }
}

static HELLO_WORLD: &str = "Hello, World!";

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    unsafe {(
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    )}
}