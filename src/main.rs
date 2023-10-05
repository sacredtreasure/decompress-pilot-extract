use std::env::Args;
use std::fs;
use std::io;
use std::path;

fn main() {
    std::process::exit(compress_());
}
fn compress_() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    let filename = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&filename).unwrap();
    let mut arch = zip::ZipArchive::new(file).unwrap();

    //loop thru every file
    for i in 0..arch.len() {
        let mut file = arch.by_index(i).unwrap();

        let outerpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            //checking for comments
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        };
            if (*file.name()).ends_with('/') {
                println!("File {} extracted to \"{}\"", i, outerpath.display());
                fs::create_dir_all(&outerpath).unwrap();
            } else {
                println!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    outerpath.display(),
                    file.size(),
                );
                if let Some(p) = outerpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    };
                }
                let mut  outerfile = fs::File::create(&outerpath).unwrap();
                io::copy(&mut file, &mut outerfile).unwrap();
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outerpath, fs::Permissions::from_mode(mode)).unwrap();
                }
        }
    }
    0
}
