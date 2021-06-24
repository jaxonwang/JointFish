use crate::allele::RawSpan;
use crate::allele::Span;
use rust_htslib::bcf;
use rust_htslib::bcf::Read;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

type ContigName = String;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Contig {
    name: ContigName,
    id: u32,
    length: u32,
}

impl Contig {
    pub fn new(name: ContigName, id: u32, length: u32) -> Self {
        Contig { name, id, length }
    }
}

#[derive(Default, Debug)]
pub struct BcfData {
    pub bcf_files: Vec<PathBuf>,
    pub bcf_readers: Vec<Option<bcf::Reader>>,
    pub contigs: HashMap<ContigName, Contig>,
    pub contig_by_file_index: HashMap<ContigName, Vec<usize>>,
}

impl BcfData {
    pub fn new(bcf_files: Vec<PathBuf>) -> Self {
        let mut ret = BcfData {
            bcf_files,
            bcf_readers: vec![],
            ..Default::default()
        };
        for _ in 0..ret.bcf_files.len() {
            ret.bcf_readers.push(None);
        }
        ret
    }

    fn open_file_with_index(&mut self, index: usize) {
        assert!(self.bcf_readers[index].is_none());
        self.bcf_readers[index] = Some(bcf::Reader::from_path(&self.bcf_files[index]).unwrap());
    }

    fn open_all(&mut self) {
        for i in 0..self.bcf_readers.len() {
            if self.bcf_readers[i].is_none() {
                self.open_file_with_index(i)
            }
        }
    }

    // open all bcfs and find contigs
    pub fn find_contigs_in_all_bcf(&mut self, ranges: &Option<Vec<RawSpan>>) {
        self.open_all();
        self.find_contigs(&ranges);
    }

    fn message_with_bcf_location(&self, msg: &str, idx: usize, line: usize) -> String {
        format!(
            "{} in bcf: {}, line {}",
            msg,
            self.bcf_files[idx].display(),
            line
        )
    }

    pub fn find_contigs(&mut self, ranges: &Option<Vec<RawSpan>>) {
        let maybe_chroms = ranges.as_ref().map(|set| {
            set.iter()
                .map(|span| span.rid.clone())
                .collect::<HashSet<_>>()
        });

        for (i, maybe_reader) in self.bcf_readers.iter().enumerate() {
            let reader = match maybe_reader {
                Some(r) => r,
                None => continue,
            };
            let records = reader.header().header_records();

            let mut found = 0;

            let field_not_found = |this: &BcfData, name: &str, idx: usize, line: usize| -> String {
                this.message_with_bcf_location(&format!("{} not found", name), idx, line)
            };

            for (line, record) in records.iter().enumerate() {
                match record {
                    bcf::header::HeaderRecord::Contig { values, .. } => {
                        let name = values
                            .get("ID")
                            .expect(&field_not_found(self, "ID", i, line));

                        // filtered by range
                        if let Some(ranges) = maybe_chroms.as_ref() {
                            if !ranges.contains(name) {
                                continue;
                            }
                        }

                        let length = values
                            .get("length")
                            .expect(&field_not_found(self, "length", i, line));
                        let length = length
                            .parse::<u32>()
                            .expect(&self.message_with_bcf_location(
                                "can not parse length",
                                i,
                                line,
                            ));

                        if self.contigs.contains_key(name) {
                            assert_eq!(
                                self.contigs[name].length,
                                length,
                                "{}",
                                self.message_with_bcf_location(
                                    &format!(
                                        "config length doesn't match: new: {:?} old: {:?}",
                                        length, self.contigs[name].length
                                    ),
                                    i,
                                    line
                                ),
                            );
                            // update file index
                            self.contig_by_file_index.get_mut(name).unwrap().push(i);
                        } else {
                            let id = self.contigs.len() as u32;
                            let new_ctg = Contig::new(name.clone(), id, length);
                            self.contigs.insert(name.clone(), new_ctg.clone());
                            self.contig_by_file_index.insert(name.clone(), vec![i]);
                        }

                        found += 1;
                    }
                    _ => (),
                }
            }
            if found == 0 {
                panic!(
                    "no contig metadata found in {}",
                    self.bcf_files[i].display()
                )
            }
            println!(
                "found {} config record in {}",
                found,
                self.bcf_files[i].display()
            )
        }
    }
}
