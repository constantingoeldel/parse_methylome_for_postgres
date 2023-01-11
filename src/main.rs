use std::error::Error;
use std::fs;
use std::io::BufRead;

fn main() {
    let files =
        std::fs::read_dir("/home/cgoeldel/epigenomics/methylome/MA3_new_total_original_methylome")
            .expect("SDSS");
    let x = [
        (0, 0),
        (1, 2),
        (1, 8),
        (2, 2),
        (2, 8),
        (4, 2),
        (4, 8),
        (5, 2),
        (5, 8),
        (8, 2),
        (8, 8),
        (11, 2),
        (11, 8),
    ];
    for (i, file) in files.enumerate() {
        let mut output = String::new();
        let path = file.unwrap().path();

        // Create buffer reader for line by line reading
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        for (n, line) in reader.lines().skip(1).enumerate() {
            if n % 10000000 == 0 {
                println!("{n}")
            }
            let line = line.unwrap();
            let r = parse(&line, x[i]);
            match r {
                Ok(s) => output += &s,
                Err(e) => println!("{line}: {e}"),
            }
        }
        // Write output to file /home/cgoeldel/epigenomics/methylome/complete
        fs::write(
            format!("/home/cgoeldel/epigenomics/methylome/complete_{i}"),
            output,
        )
        .unwrap();
    }
}

fn parse(line: &str, x: (i32, i32)) -> Result<String, Box<dyn Error>> {
    let mut line = line.split('\t');
    let seqname = line.next().unwrap().parse::<u32>()?;
    let start = line.next().unwrap().parse::<u32>()?;
    let strand = line.next().unwrap() == "+";
    let context = line.next().unwrap();
    let methylated = line.next().unwrap().parse::<u32>()?;
    let total = line.next().unwrap().parse::<u32>()?;
    let posterior_max = line.next().unwrap().parse::<f32>()?;
    let status = line.next().unwrap();
    let rc = line.next().unwrap().parse::<f32>()?;

    Ok(format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        seqname, start, strand, context, methylated, total, posterior_max, status, rc, x.0, x.1
    ))
}
