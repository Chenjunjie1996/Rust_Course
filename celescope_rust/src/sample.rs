#![allow[unused,warnings]]
use std::collections::Hashmap;
use std::process::Command;

fn add_kit_version(chemistry: String) -> String {
    let kit_list = vec![
        ("1", "no longer in use"),
        ("2", "kit V1"),
        ("3", "kit V2"),
    ];
    let kit_map= kit_list.into_iter().collect();

    if chemistry.starts_with("scopeV") {
        let s = chemistry.replace("scopeV", "");
        let chem_version = &s[0];
        let kit = kit_map.get(chem_version);
        return chemistry + "(" + kit + ")"
    }
    chemistry
}

struct Sample {
    sample: String,
    outdir: String,
    assay_description: String,
    version: String,
    chemistry: String,
}

trait Run{
    fn run(&self) {}
}

impl Run for Sample {
    fn run(&self) {
        match self.chemistry {
            String::from("auto") => {
                "chemistry"
            }
            _ => &self.chemistry
        }
        let metric_dict = self.add_metric();
    }
}

trait Add_metric{
    fn add_metric(&self) -> Hashmap{}
}

impl Add_metric for Sample  {
    fn add_metric(&self) {
        let mut metric_dict = Hashmap::new();
        metric_dict.insert("Chemistry".to_string(), self.chemistry);
        metric_dict.insert("Assay".to_string(), self.assay_description);
        metric_dict.insert("Software Version".to_string(), self.version);
        metric_dict
    }
}



fn main() {
    let sample = Sample{
        sample: todo!(),
        outdir: todo!(),
        assay_description: todo!(),
        version: todo!(),
        chemistry: todo!(),
    };
    sample.run();
}