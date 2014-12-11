extern crate "rust-crypto" as rust_crypto;
extern crate getopts;

use getopts::{reqopt,optflag,getopts,OptGroup};

use std::os;
use std::rand::{OsRng, Rng};
use std::io::File;
use std::str;

use rust_crypto::pbkdf2::pbkdf2;
use rust_crypto::sha2::Sha256;
use rust_crypto::digest::Digest;
use rust_crypto::md5::Md5;
use rust_crypto::mac::Mac;
use rust_crypto::hmac::Hmac;

struct Mnemonic {
    words: Vec<u8>
}

//getopts help message
fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-s\t\tSeed");
    println!("-h --help\tUsage");
}

fn main() {
    /* start handling opts */
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    let opts = &[
        reqopt("s", "seed", "set mnemonic seed", ""),
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }
    let seed = match matches.opt_str("s") {
        Some(x) => x,
        None => panic!("No seed given"),
    };
    /* end opts, seed value below */
    let str_seed:&str = seed.as_slice();

    println!("{}", str_seed);
    println!("md5: {}", gen_md5(str_seed));
    println!("sha256: {}", gen_sha256(str_seed));

    let mut rng = match OsRng::new() {
      Ok(g) => g,
      Err(e) => panic!("Failed to obtain OS RNG: {}", e)
    };

    let num:u32 = rng.next_u32();
    println!("{}",num);

    let path = Path::new("src/wordslist/english.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.desc),
        Ok(file) => file,
    };
    let words:String = match file.read_to_string() {
        Err(why) => panic!("couldn't read {}: {}", display, why.desc),
        Ok(string) => string,
    };
    // for word in words.words() {
    //     println!("{}",word)
    // }

    println!("{}",words.words().count());
    //generate corner cases
    for &i in [16u,24u,32u].iter() {
        for n in ["00","7f","80","ff"].iter() {
            println!("{}",n.repeat(i))
        }
    }
    // for gen_seed in range(0,12u) {
    //
    // }

}

fn gen_md5(hashme:&str) -> String {
    let mut sh = Md5::new();
    sh.input_str(hashme);

    sh.result_str()
}

fn gen_sha256(hashme:&str) -> String {
    let mut sh = Sha256::new();
    sh.input_str(hashme);

    sh.result_str()
}
