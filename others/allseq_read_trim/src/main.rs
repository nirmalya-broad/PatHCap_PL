#[macro_use]
extern crate clap;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::File;
use std::string::String;

#[allow(non_snake_case)]
fn process_fastq(infile:&str, outfile:&str, sfile:&str, con_a:usize, read_len:usize) -> () {
    let f = File::open(infile).expect("Unable to open input file");
    let mut reader = BufReader::new(f);

    let f_out = File::create(outfile).expect("Unable to create output file");
    let s_out = File::create(sfile).expect("Unable to create survival file");
    let mut writer = BufWriter::new(f_out);
    let mut s_writer = BufWriter::new(s_out);


    let mut lcount = 0;

    loop {
        let mut line1 = String::new();
        let mut line2 = String::new();
        let mut line3 = String::new();
        let mut line4 = String::new();

        let num_bytes1 = reader.read_line(&mut line1)
            .expect("Failed to read line1");
        if num_bytes1 == 0 {
            println!("End of file. Breaking the loop.");
            break;
        }

        let num_bytes2 = reader.read_line(&mut line2)
            .expect("Failed to read line2");
        if num_bytes2 == 0 {
            panic!("Incomplete file while reading line 2!");
        }

        let num_bytes3 = reader.read_line(&mut line3)
            .expect("Failed to read line3");
        if num_bytes3 == 0 {
            panic!("Incomplete file while reading line 3!");
        }

        let num_bytes4 = reader.read_line(&mut line4)
            .expect("Failed to read line4");
        if num_bytes4 == 0 {
            panic!("Incomplete file while reading line 4!");
        }
        lcount += 1;

        // Remove the rightmost newline character
        line1.pop();
        line2.pop();
        line3.pop();
        line4.pop();

        let parts_vec = line1.split_whitespace().collect::<Vec<&str>>();
                
        let len = line2.chars().count();

        let mut A_count = 0;
        // We adjust last_end as one based position.
        let mut last_end  = 0;
        for (i, lchar) in line2.chars().enumerate() {
            if lchar == 'A' {
                A_count += 1;
                if A_count == con_a {
                    // Time to trim
                    break;
                }
            } else {
                // Reset
                A_count = 0;
                last_end = i + 1;
            }
        }

        if last_end >= read_len {   
            let line1_fmt = get_fmt_line1(parts_vec, len, last_end);
            let line2_fmt = line2.chars().take(last_end).collect::<String>();
            let line3_fmt = line3;
            let line4_fmt = line4.chars().take(last_end).collect::<String>();
            writeln!(&mut writer, "{}", line1_fmt).unwrap();
            writeln!(&mut writer, "{}", line2_fmt).unwrap();
            writeln!(&mut writer, "{}", line3_fmt).unwrap();
            writeln!(&mut writer, "{}", line4_fmt).unwrap();

            let sline: String = lcount.to_string();
            writeln!(&mut s_writer, "{}", sline).unwrap();
        }

        // Process the third

        if lcount % 1000000 == 0 {
            println!("lcount: {}", lcount);
        }
    }

    println!("Total count of lines in infile: {}", lcount * 4)

}

fn get_fmt_line1(parts_vec:Vec<&str>, len: usize, last_end: usize) -> String {
    let out_str0;
    if len != last_end {
        out_str0 = format!("{}:trimmed_{}_{}", parts_vec[0], (last_end + 1), (len - last_end));
    } else {
        out_str0 = format!("{}:untrimmed_0_0", parts_vec[0]);
    }
    let out_str;
    if parts_vec.len() == 1 {
        out_str = out_str0;
    } else {
        out_str = format!("{} {}", out_str0, parts_vec[1]);
    }
    out_str

}

fn main() {
    let con_a_def = "7";
    let read_len_def = "20";

    let matches = clap_app!(myapp =>
        (about: "Does awesome things")
        (@arg Infile: -i --infile +takes_value +required "Input file")
        (@arg Outfile: -o --outfile +takes_value +required "Output file")
        (@arg Sfile: -s --Sfile +takes_value "Survival file")
        (@arg Con_A: -c --con_A +takes_value  "Minimum num of consecutive A to start trimming")
        (@arg Read_len: -r --read_len +takes_value "Min read len required to keep")
    ).get_matches();

    let infile;
    if matches.is_present("Infile") {
        infile = matches.value_of("Infile").unwrap();
        println!("value of infile: {}", infile);
    } else {
        panic!("value of infile not present");
    }

    let outfile;
    if matches.is_present("Outfile") {
        outfile = matches.value_of("Outfile").unwrap();
        println!("value of outfile: {}", outfile);
    } else {
        panic!("value of outfile not present");
    }

    let sfile_def = str::replace(outfile, ".fastq", ".sfile");
    let sfile = matches.value_of("Sfile").unwrap_or(&sfile_def);
    println!("value of Sfile: {}", sfile);

    let con_a:usize;
    let con_a_obj = matches.value_of("Con_A").unwrap_or(con_a_def);
    con_a = con_a_obj.parse::<usize>().unwrap();
    println!("value of Con_a : {}", con_a);

    let read_len:usize;
    let read_len_obj = matches.value_of("Read_len").unwrap_or(read_len_def);
    read_len = read_len_obj.parse::<usize>().unwrap();
    println!("value of Read_len : {}", read_len);

    process_fastq(infile, outfile, &sfile, con_a, read_len)



}
