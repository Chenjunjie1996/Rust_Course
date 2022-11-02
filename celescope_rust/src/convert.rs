
// 项目(Packages)：一个 Cargo 提供的 feature，可以用来构建、测试和分享包
// 包(Crate)：一个由多个模块组成的树形结构，可以作为三方库进行分发，也可以生成可执行文件进行运行
// 模块(Module)：可以一个文件多个模块，也可以一个文件一个模块，模块可以被认为是真实项目中的代码组织单元

use std::collections::{HashMap, hash_map};
use std::fs::{File, read_to_string};
use std::ops::Range;
use std::process::Command;
use serde::{Deserialize, Serialize};

const TSO: &str = "TTTCTTATATGGG";
const UMI_LEN: usize = 10;
const WHITELIST_SUFFIX_OLD: &str = "/cellranger-cs/3.0.2/lib/python/cellranger/barcodes/737K-august-2016.txt";
const WHITELIST_SUFFIX_NEW: &str = "/lib/python/cellranger/barcodes/737K-august-2016.txt";

struct Convert{
    sample: String,
    outdir: String,
    in_fq2: String,
    out_fq1_file: String,
    out_fq2_file: String,
    barcode_convert_hashmap: String,
}

impl Convert{
    fn write_fq1(&self) {
        let out_fq1 = File::create(&self.out_fq1_file);
    }
    // write out fq2
    fn write_fq2(&self) {
        let cmd = "gzip -c".to_string() + &self.in_fq2 + ">" + &self.out_fq2_file;
        let output = Command::new("sh").arg("-c").arg(cmd).output().expect("sh exec error!");
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);
    }
    // write json
    fn dump_tenx_sgr_barcode_json(&self) {
        let mut sgr_tenx:HashMap<String, String> = HashMap::new();
        sgr_tenx.insert("1".to_string(), "2".to_string());
        let mut tenx_sgr:HashMap<String, String> = HashMap::new();

        for (k, v) in &sgr_tenx {
            tenx_sgr.insert(v.clone(), k.clone());
        }
        let json = serde_json::to_string(&tenx_sgr).unwrap();
    }
}

fn convert_seq(sgr_tenx: HashMap<String, String>, sgr_barcode: &mut String, sgr_umi:&mut String) -> (String, String){
    let barcode_10x = {
        if sgr_tenx.contains_key(sgr_barcode){
            sgr_tenx.get(sgr_barcode).unwrap().to_string()
        }else {
            let barcode_10x = WHITELIST_SUFFIX_NEW.lines().next().unwrap().to_string();
            sgr_tenx.insert(*sgr_barcode, barcode_10x);
            barcode_10x
        }
    };
    
    let umi_len_sgr = sgr_umi.len();
    let umi_10x = {
        if umi_len_sgr > UMI_LEN {
            &sgr_umi[0..UMI_LEN]
        } else if umi_len_sgr < UMI_LEN {
            *sgr_umi += "C";
            sgr_umi
        } else {
            sgr_umi
        }
    };
    let new_seq1 = barcode_10x.clone() + umi_10x + TSO;
    let mut new_qual1 = String::new();
    for i in 0..new_seq1.len(){
        new_qual1.push_str("F");
    };
    (new_seq1, new_qual1)
}

fn run(class: Convert) {
    class.write_fq1();
    class.write_fq2();
    class.dump_tenx_sgr_barcode_json();
}

fn main() {
    let convert = Convert{
        sample: "test".to_string(),
        outdir: "/SGRNJ06/randd/USER/cjj/celedev".to_string(),
        in_fq2: "/SGRNJ06/randd/USER/cjj/celedev/test.fq".to_string(),
        out_fq1_file: "/SGRNJ06/randd/USER/cjj/celedev/out_fq1".to_string(),
        out_fq2_file: "/SGRNJ06/randd/USER/cjj/celedev/out_fq2".to_string(),
        barcode_convert_hashmap: "/SGRNJ06/randd/USER/cjj/celedev/barcode_convert.json".to_string(),
    };
    run(convert);
}
