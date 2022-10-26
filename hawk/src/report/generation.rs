use std::{collections::HashMap, fs};

use crate::analyzer::{
    optimizations::Optimization, qa::QualityAssurance, vulnerabilities::Vulnerability,
};

use super::{
    optimization_report::generate_optimization_report, qa_report::generate_qa_report,
    vulnerability_report::generate_vulnerability_report,
};

pub fn generate_report(
    vulnerabilities: HashMap<Vulnerability, Vec<(String, Vec<i32>)>>,
    optimizations: HashMap<Optimization, Vec<(String, Vec<i32>)>>,
    qa: HashMap<QualityAssurance, Vec<(String, Vec<i32>)>>,
) {
    let mut hawk_report = String::from("");

    if vulnerabilities.len() > 0 {
        hawk_report.push_str(&generate_vulnerability_report(vulnerabilities));
        hawk_report.push_str("\n\n");
    }

    if optimizations.len() > 0 {
        hawk_report.push_str(&generate_optimization_report(optimizations));
        hawk_report.push_str("\n\n");
    }

    if qa.len() > 0 {
        hawk_report.push_str(&generate_qa_report(qa));
        hawk_report.push_str("\n\n");
    }

    fs::write("hawk_report.md", hawk_report).expect("Unable to hawk_report to file");
}
