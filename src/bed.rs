use crate::allele::Span;
use std::path::Path;
use bio::io::bed;
use std::convert::TryInto;
use std::fs::File;
use crate::allele::RawSpan;

pub fn ranges_from_bed(r: impl std::io::Read) -> Vec<RawSpan> {
    let mut r = bed::Reader::new(r);
    let mut ret = vec![];
    for record in r.records() {
        let record = record.expect("bad bed file");
        let s = RawSpan::new(
            record.chrom().to_string(),
            record.start().try_into().unwrap(),
            record.end().try_into().unwrap(),
        );
        ret.push(s);
    }
    ret
}

pub fn range_from_bed_file<P: AsRef<Path>>(path: P) -> Vec<RawSpan> {
    let f = File::open(path).unwrap();
    ranges_from_bed(f)
}
