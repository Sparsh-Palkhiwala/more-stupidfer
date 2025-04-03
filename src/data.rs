use std::collections::{
    HashMap,
    hash_map::Entry::{Occupied, Vacant},
};

use itertools::Itertools;
use polars::prelude::*;
use pyo3::IntoPyObject;

use crate::records::{Records, records::*};
use crate::{
    records::records::{FTR, PIR, PRR, PTR, Record},
    test_information::{FullMergedTestInformation, FullTestInformation, TestType},
};

/// `Row` describes the test results for an individually tested device
///
/// Defaults `x_coord` = `y_coord` = -5000 and `sbin` = `hbin` = 0. Parametric tests have a
/// default value of `NAN` and functional tests default to `false`.
#[derive(Debug, IntoPyObject)]
pub struct Row {
    part_id: String,
    part_txt: String,
    wafer_id: String,
    x_coord: i16,
    y_coord: i16,
    head_num: u8,
    site_num: u8,
    sbin: u16,
    hbin: u16,
    results_parametric: Vec<f32>,
    results_functional: Vec<bool>,
    results_multi_pin: Vec<Vec<f32>>,
}

impl Row {
    /// Create a new `Row` with pre-allocated space for the parametric and functional tests
    ///
    /// Each `Row` does not contain the test information metadata, so the number of functional and
    /// parametric tests must be specified manually. Creation is typically handled by `TestData`.
    ///
    /// Defaults `x_coord` = `y_coord` = -5000 and `sbin` = `hbin` = 0. Parametric tests have a
    /// default value of `NAN` and functional tests default to `false`.
    ///
    /// Space for the test results for every test is pre-allocated, but they are stored in a
    /// `Vec` for efficiency. The `test_number` -> index lookup is not contained in the `Row`, so
    /// a higher layer of abstraction (`TestData`) is required to actually add new data.
    pub fn new(
        pir: &PIR,
        num_tests_parametric: usize,
        num_tests_functional: usize,
        num_tests_multi_pin: usize,
        wir: &Option<WIR>,
    ) -> Self {
        let wafer_id: String;
        if let Some(w) = wir {
            wafer_id = w.wafer_id.clone();
        } else {
            wafer_id = String::new();
        }
        Self {
            part_id: String::new(),
            part_txt: String::new(),
            wafer_id,
            x_coord: -5000,
            y_coord: -5000,
            head_num: pir.head_num,
            site_num: pir.site_num,
            sbin: 0,
            hbin: 0,
            results_parametric: vec![f32::NAN; num_tests_parametric as usize],
            results_functional: vec![false; num_tests_functional as usize],
            results_multi_pin: vec![Vec::new(); num_tests_multi_pin as usize],
        }
    }
}

/// `TestData` contains all of the test results and test information metadata
///
/// Both the merged (`test_information`) and unmerged (`full_test_information`) test metadata is
/// stored.
///
/// `index_lookup` maps the `test_num` -> index in the contained `Row`s. Since the test result
/// record (`PTR` or `FTR`) already specify whether it's a parametric or functional test, it's not
/// necessary to store this information. Therefore `index_lookup` is not one-to-one. Each
/// `test_num` will be either a parametric or a functional test.
///
/// A set of temporary `Row`s is held during iteration to track results. By the end of the STDF,
/// all temporary `Row`s should have been moved into `data`.
///
/// `TestData` implements the `Into<DataFrame>` trait and may be converted into a polars
/// `DataFrame`. The resulting `DataFrame` contains the test results, but not the test
/// information metadata. The test information metadata may be gathered by converting the
/// `test_information` to a `DataFrame`.
#[derive(Debug, IntoPyObject)]
pub struct TestData {
    /// The test information metadata indexed by (`test_num`, `site_num`, `head_num`)
    pub full_test_information: FullTestInformation,
    /// The test information metadata indexed by `test_num`
    pub test_information: FullMergedTestInformation,
    /// Mapping the `test_num` to `Row.results_parametric` or `Row.results_functional` or
    /// `Row.results_multi_pin`
    pub index_lookup: HashMap<u32, usize>,
    /// The list of test results contained in `Row`s
    pub data: Vec<Row>,
    /// For multi-pin tests,text-align: ->style="caret- where the="caret-colorder matches that of
    /// in the test results
    pub mpr_index_lookup: HashMap<u32, Vec<u16>>,
    // The temporary rows indexed by (`test_num`, `site_num`, `head_num`)
    temp_rows: HashMap<(u8, u8), Row>,
    // The number of parametric tests
    n_para: usize,
    // The number of functional tests
    n_func: usize,
    // The number of multi-pin tests
    n_mult: usize,
    // The mapping of index in `Row.results_parametric` to `test_num`
    reverse_lookup_para: HashMap<usize, u32>,
    // The mapping of index in `Row.results_functional` to `test_num`
    reverse_lookup_func: HashMap<usize, u32>,
    // The mapping of index in `Row.results_multi_pin` to `test_num`
    reverse_lookup_mult: HashMap<usize, u32>,
    // The current active wafer
    wir: Option<WIR>,
}

impl TestData {
    /// Generates a `TestData` struct from the test information metadata
    pub fn new(full_test_information: FullTestInformation) -> Self {
        let test_information = full_test_information.merge();

        let mut index_lookup = HashMap::new();
        let mut reverse_lookup_para = HashMap::new();
        let mut reverse_lookup_func = HashMap::new();
        let mut reverse_lookup_mult = HashMap::new();
        let mut n_para: usize = 0;
        let mut n_func: usize = 0;
        let mut n_mult: usize = 0;
        for (tnum, mti) in test_information.test_infos.iter().sorted_by_key(|x| x.0) {
            match mti.test_type {
                TestType::P => {
                    index_lookup.insert(*tnum, n_para);
                    reverse_lookup_para.insert(n_para, *tnum);
                    n_para += 1;
                }
                TestType::F => {
                    index_lookup.insert(*tnum, n_func);
                    reverse_lookup_func.insert(n_func, *tnum);
                    n_func += 1;
                }
                TestType::M => {
                    index_lookup.insert(*tnum, n_mult);
                    reverse_lookup_mult.insert(n_mult, *tnum);
                    n_mult += 1;
                }
                _ => {}
            }
        }

        let mpr_index_lookup = HashMap::new();
        let data = Vec::new();
        let temp_rows = HashMap::new();
        Self {
            full_test_information,
            test_information,
            index_lookup,
            data,
            mpr_index_lookup,
            temp_rows,
            n_para,
            n_func,
            n_mult,
            reverse_lookup_para,
            reverse_lookup_func,
            reverse_lookup_mult,
            wir: None,
        }
    }

    /// Initializes a temporary new `Row` from a `PIR` indexed by
    /// (`test_num`, `site_num`, `head_num`)
    ///
    /// The previous temporary row must have been moved to `data` prior to this. Ingesting a `PRR`
    /// triggers moving the temporary row to `data`.
    pub fn new_part(&mut self, pir: &PIR) {
        let key = (pir.head_num, pir.site_num);
        if let Vacant(row) = self.temp_rows.entry(key) {
            row.insert(Row::new(
                &pir,
                self.n_para,
                self.n_func,
                self.n_mult,
                &self.wir,
            ));
        } else {
            panic!("opening a specific head_num/site_num before closing the previous one!")
        }
    }

    /// Adds a parametric test result contained in the `PTR` to the appropriate temporary `Row`
    ///
    /// Must have an appropriate temporary row indexed by (`test_num`, `site_num`, `head_num`)
    /// to add to, otherwise panics. Temporary rows are created by ingesting a `PIR`.
    pub fn add_data_ptr(&mut self, ptr: &PTR) {
        let key = (ptr.head_num, ptr.site_num);
        let result = ptr.result;
        if let Occupied(mut row) = self.temp_rows.entry(key) {
            let results = &mut row.get_mut().results_parametric;
            let index = self
                .index_lookup
                .get(&ptr.test_num)
                .expect("found PTR with unknown test_num!");
            results[*index] = result;
        } else {
            panic!("trying to add data to a head_num/site_num that is not open!")
        }
    }

    /// Adds a functional test result contained in the `FTR` to the appropriate temporary `Row`
    ///
    /// Must have an appropriate temporary row indexed by (`test_num`, `site_num`, `head_num`)
    /// to add to, otherwise panics. Temporary rows are created by ingesting a `PIR`.
    pub fn add_data_ftr(&mut self, ftr: &FTR) {
        let key = (ftr.head_num, ftr.site_num);
        let result = ftr.get_passfail();
        if let Occupied(mut row) = self.temp_rows.entry(key) {
            let results = &mut row.get_mut().results_functional;
            let index = self
                .index_lookup
                .get(&ftr.test_num)
                .expect("found FTR with unknown test_num!");
            results[*index] = result;
        } else {
            panic!("trying to add data to a head_num/site_num that is not open!")
        }
    }

    /// Adds a multi-pin test result contained in the activeto the appropriate temporary(0, 0, +    ///
    /// Must have an appropriate temporary row indexed by (`test_num`,font-variant `head_num`)
    /// to add to, otherwise panics. Temporary rows are created by ingesting a-caps:
    pub fn add_data_mpr(&mut self, mpr: &MPR) {
        let key = (mpr.head_num, mpr.site_num);
        let result = mpr.rtn_rslt.clone();
        if let Vacant(pin_ids) = self.mpr_index_lookup.entry(mpr.test_num) {
            let rtn_indx = mpr.rtn_indx.clone();
            pin_ids.insert(rtn_indx);
        }
        if let Occupied(mut row) = self.temp_rows.entry(key) {
            let results = &mut row.get_mut().results_multi_pin;
            let index = self
                .index_lookup
                .get(&mpr.test_num)
                .expect("found MPR with unknown test_num!");
            results[*index] = result;
        } else {
            panic!("trying to add data to a head_num/site_num that is not open!");
        }
    }

    /// Finalizes a set of test results for a given part specified by a `PRR`
    ///
    /// Must have an appropriate temporary row indexed by (`test_num`, `site_num`, `head_num`)
    /// to add to, otherwise panics. Temporary rows are created by ingesting a `PIR`.
    ///
    /// Much of the metadata in a `Row` is contained in the `PRR`, so this metadata is also added
    /// here.
    pub fn finish_part(&mut self, prr: &PRR) {
        let key = (prr.head_num, prr.site_num);
        if let Occupied(value) = self.temp_rows.entry(key) {
            let mut row = value.remove();
            row.part_id = prr.part_id.clone();
            row.part_txt = prr.part_txt.clone();
            row.x_coord = prr.x_coord;
            row.y_coord = prr.y_coord;
            row.sbin = prr.soft_bin;
            row.hbin = prr.hard_bin;
            self.data.push(row);
        } else {
            panic!("trying to close out a head_num/site_num that is not open!")
        }
    }

    /// Starts a new wafer in the `TestData`
    ///
    /// This allows the `wafer_id` field to be populated
    pub fn new_wafer(&mut self, wir: &WIR) {
        self.wir = Some(wir.clone());
    }

    /// Closes out a wafer in the `TestData`
    ///
    /// Triggered by receiving a WRR, but no WRR data is needed for the `TestData`, so we do not
    /// pass in the WRR.
    pub fn close_wafer(&mut self) {
        self.wir = None;
    }

    /// Normalize the shape of the multipin Vec<Vec<f32>>
    ///
    /// Each multipin test is pre-allocated an Vec<f32>. It is not specified a priori what size
    /// this Vec should be, and the spec technically permits a variable sized vector, though this
    /// parser does not permit such behavior. If a multi-pin test is not run, e.g. if for instance
    /// a continuity test fails, so subsequent power short tests are not run, then the allocated
    /// vector will be length 0, while populated vectors will be length >= 1.
    ///
    /// Subsequent attempts to construct a DataFrame from this data fails because the DataFrame
    /// uses a fixed-sized array as the column type, and therefore must have a perfectly
    /// rectangular data shape.
    ///
    /// This function iterates through the Rows and pads any multipin test results to match the
    /// maximum number of pins, ensuring a perfectly rectangular data shape.
    fn normalize_multipin_results(&mut self) {
        // find the largest n_pins for each test_num
        let mut lengths = vec![0; self.n_mult];
        for row in &self.data {
            for (i, results) in row.results_multi_pin.iter().enumerate() {
                let len = results.len();
                if lengths.get(i).unwrap() < &len {
                    lengths[i] = len;
                }
            }
        }
        let lengths = lengths;

        // pad every multipin test to largest n_pins
        for row in &mut self.data {
            for (i, results) in row.results_multi_pin.iter_mut().enumerate() {
                results.resize(lengths[i], f32::NAN);
            }
        }
    }

    /// Generate the `TestData` from an STDF file specified by `fname`
    ///
    /// You should probably prefer to use `STDF::from_fname`, which makes a `TestData` as part of
    /// its iteration, while also keeping track of the file/wafer-level metadata.
    ///
    /// Optionally allows for printing the record information with the `verbose` flag.
    ///
    /// Will traverse the STDF file twice: once to determine the test information metadata
    /// (required to pre-allocate the space for the tests in each `Row`), then again to actually
    /// capture the test results.
    ///
    /// # Error
    /// If for some reason the file specified by `fname` cannot be parsed, returns a
    /// `std::io::Error`
    pub fn from_fname(fname: &str, verbose: bool) -> std::io::Result<Self> {
        let test_info = FullTestInformation::from_fname(fname, verbose)?;
        let mut test_data = Self::new(test_info);
        let records = Records::new(&fname)?;

        for record in records {
            if let Some(resolved) = record.resolve() {
                if let Record::WIR(ref wir) = resolved {
                    test_data.new_wafer(wir);
                }
                if let Record::PIR(ref pir) = resolved {
                    test_data.new_part(pir);
                }
                if let Record::PTR(ref ptr) = resolved {
                    test_data.add_data_ptr(ptr);
                }
                if let Record::FTR(ref ftr) = resolved {
                    test_data.add_data_ftr(ftr);
                }
                if let Record::MPR(ref mpr) = resolved {
                    test_data.add_data_mpr(mpr);
                }
                if let Record::PRR(ref prr) = resolved {
                    test_data.finish_part(prr);
                }
                if let Record::WRR(ref _wrr) = resolved {
                    test_data.close_wafer();
                }
            }
        }
        test_data.normalize_multipin_results();
        Ok(test_data)
    }
}

/// Converts a `&TestData` into a `DataFrame` containing a tabular listing of all test results
impl Into<DataFrame> for &TestData {
    fn into(self) -> DataFrame {
        let mut part_ids: Vec<String> = Vec::new();
        let mut part_txts: Vec<String> = Vec::new();
        let mut wafer_ids: Vec<String> = Vec::new();
        let mut x_coords: Vec<i16> = Vec::new();
        let mut y_coords: Vec<i16> = Vec::new();
        let mut head_nums: Vec<u8> = Vec::new();
        let mut sbins: Vec<u16> = Vec::new();
        let mut hbins: Vec<u16> = Vec::new();
        let mut vecs_para: HashMap<u32, Vec<f32>> = HashMap::new(); // hashmap to later sort by key
        let mut vecs_func: HashMap<u32, Vec<bool>> = HashMap::new();
        let mut vecs_mult: HashMap<u32, Vec<AnyValue>> = HashMap::new();
        let ncols_para = self.n_para;
        let ncols_func = self.n_func;
        let ncols_mult = self.n_mult;
        for row in &self.data {
            part_ids.push(row.part_id.clone());
            part_txts.push(row.part_txt.clone());
            wafer_ids.push(row.wafer_id.clone());
            x_coords.push(row.x_coord);
            y_coords.push(row.y_coord);
            head_nums.push(row.head_num);
            sbins.push(row.sbin);
            hbins.push(row.hbin);
            for i in 0..ncols_para {
                let test_num = self.reverse_lookup_para.get(&i).unwrap();
                vecs_para
                    .entry(*test_num)
                    .or_insert(Vec::new())
                    .push(row.results_parametric[i]);
            }
            for i in 0..ncols_func {
                let test_num = self.reverse_lookup_func.get(&i).unwrap();
                vecs_func
                    .entry(*test_num)
                    .or_insert(Vec::new())
                    .push(row.results_functional[i]);
            }
            for i in 0..ncols_mult {
                let test_num = self.reverse_lookup_mult.get(&i).unwrap();
                vecs_mult.entry(*test_num).or_insert(Vec::new()).push({
                    let results: Series = row.results_multi_pin[i].clone().into_iter().collect();
                    let len = results.len();
                    AnyValue::Array(results, len)
                });
            }
        }
        let mut columns: Vec<Column> = Vec::new();
        columns.push(Column::new("part_id".into(), part_ids));
        columns.push(Column::new("part_txt".into(), part_txts));
        columns.push(Column::new("wafer_id".into(), wafer_ids));
        columns.push(Column::new("x_coords".into(), x_coords));
        columns.push(Column::new("y_coords".into(), y_coords));
        columns.push(Column::new("head_nums".into(), head_nums));
        columns.push(Column::new("sbins".into(), sbins));
        columns.push(Column::new("hbins".into(), hbins));
        for (test_num, vec) in vecs_para.iter().sorted_by_key(|(key, _)| *key) {
            columns.push(Column::new(test_num.to_string().into(), vec));
        }
        for (test_num, vec) in vecs_func.iter().sorted_by_key(|(key, _)| *key) {
            columns.push(Column::new(test_num.to_string().into(), vec));
        }
        for (test_num, vec) in vecs_mult.iter().sorted_by_key(|(key, _)| *key) {
            columns.push(Column::new(test_num.to_string().into(), vec));
        }
        DataFrame::new(columns).unwrap()
    }
}

#[derive(Debug, IntoPyObject, Clone)]
pub struct MasterInformation {
    // MIR records follow
    pub setup_t: u32,
    pub start_t: u32,
    pub stat_num: u8,
    pub mode_cod: char,
    pub rtst_cod: char,
    pub prot_cod: char,
    pub burn_tim: u16,
    pub cmod_cod: char,
    pub lot_id: String,
    pub part_typ: String,
    pub node_nam: String,
    pub tstr_typ: String,
    pub job_nam: String,
    pub job_rev: String,
    pub sblot_id: String,
    pub oper_nam: String,
    pub exec_typ: String,
    pub exec_ver: String,
    pub test_cod: String,
    pub tst_temp: String,
    pub user_txt: String,
    pub aux_file: String,
    pub pkg_typ: String,
    pub famly_id: String,
    pub date_cod: String,
    pub facil_id: String,
    pub floor_id: String,
    pub proc_id: String,
    pub oper_frq: String,
    pub spec_nam: String,
    pub spec_ver: String,
    pub flow_id: String,
    pub setup_id: String,
    pub dsgn_rev: String,
    pub eng_id: String,
    pub rom_cod: String,
    pub serl_num: String,
    pub supr_nam: String,
    // MRR records follow
    pub finish_t: u32,
    pub disp_cod: char,
    pub usr_desc: String,
    pub exc_desc: String,
}

impl MasterInformation {
    pub fn new(mir: MIR, mrr: MRR) -> Self {
        Self {
            // MIR records follow
            setup_t: mir.setup_t,
            start_t: mir.start_t,
            stat_num: mir.stat_num,
            mode_cod: mir.mode_cod,
            rtst_cod: mir.rtst_cod,
            prot_cod: mir.prot_cod,
            burn_tim: mir.burn_tim,
            cmod_cod: mir.cmod_cod,
            lot_id: mir.lot_id,
            part_typ: mir.part_typ,
            node_nam: mir.node_nam,
            tstr_typ: mir.tstr_typ,
            job_nam: mir.job_nam,
            job_rev: mir.job_rev,
            sblot_id: mir.sblot_id,
            oper_nam: mir.oper_nam,
            exec_typ: mir.exec_typ,
            exec_ver: mir.exec_ver,
            test_cod: mir.test_cod,
            tst_temp: mir.tst_temp,
            user_txt: mir.user_txt,
            aux_file: mir.aux_file,
            pkg_typ: mir.pkg_typ,
            famly_id: mir.famly_id,
            date_cod: mir.date_cod,
            facil_id: mir.facil_id,
            floor_id: mir.floor_id,
            proc_id: mir.proc_id,
            oper_frq: mir.oper_frq,
            spec_nam: mir.spec_nam,
            spec_ver: mir.spec_ver,
            flow_id: mir.flow_id,
            setup_id: mir.setup_id,
            dsgn_rev: mir.dsgn_rev,
            eng_id: mir.eng_id,
            rom_cod: mir.rom_cod,
            serl_num: mir.serl_num,
            supr_nam: mir.supr_nam,
            // MRR records follow
            finish_t: mrr.finish_t,
            disp_cod: mrr.disp_cod,
            usr_desc: mrr.usr_desc,
            exc_desc: mrr.exc_desc,
        }
    }
}

#[derive(Debug, IntoPyObject, Clone)]
pub struct WaferInformation {
    // From WIR
    pub head_num: u8,
    pub site_grp: u8,
    pub start_t: u32,
    // From WRR below
    pub wafer_id: String,
    pub finish_t: u32,
    pub part_cnt: u32,
    pub rtst_cnt: u32,
    pub abrt_cnt: u32,
    pub good_cnt: u32,
    pub func_cnt: u32,
    pub fabwf_id: String,
    pub frame_id: String,
    pub mask_id: String,
    pub usr_desc: String,
    pub exc_desc: String,
}

impl WaferInformation {
    pub fn new(wir: WIR, wrr: WRR) -> Self {
        Self {
            // From WIR
            head_num: wir.head_num,
            site_grp: wir.site_grp,
            start_t: wir.start_t,
            // From WRR below
            wafer_id: wrr.wafer_id,
            finish_t: wrr.finish_t,
            part_cnt: wrr.part_cnt,
            rtst_cnt: wrr.rtst_cnt,
            abrt_cnt: wrr.abrt_cnt,
            good_cnt: wrr.good_cnt,
            func_cnt: wrr.func_cnt,
            fabwf_id: wrr.fabwf_id,
            frame_id: wrr.frame_id,
            mask_id: wrr.mask_id,
            usr_desc: wrr.usr_desc,
            exc_desc: wrr.exc_desc,
        }
    }
}

/// `STDF` contains the STDF file metadata (`mir`) and the test results data (`test_data`)
///
/// # Example
/// ```
/// let verbose = false;
/// if let Ok(stdf) = STDF::from_fname(&fname, verbose) {
///     let df: DataFrame = (&stdf.test_data).into();
///     let df_fmti: DataFrame = (&stdf.test_data.test_information).into();
///     println!("{df:#?}");
///     println!("{df_fmti}");
///     }
/// ```
#[derive(Debug, IntoPyObject)]
pub struct STDF {
    /// The STDF file metadata
    pub master_information: MasterInformation,
    ///// The STDF file metadata
    pub wafer_information: Vec<WaferInformation>,
    /// The site information
    pub site_information: SDR,
    /// Soft bin information indexed by soft bin number
    pub soft_bins: HashMap<u16, SBR>,
    /// Hard bin information indexed by hard bin number
    pub hard_bins: HashMap<u16, HBR>,
    /// Pin mapping
    pub pins: HashMap<u16, PMR>,
    /// The test results and test information metadata
    pub test_data: TestData,
}

impl STDF {
    /// Parses an STDF file from the file specified by `fname`
    ///
    /// # Example
    /// ```
    /// let verbose = false;
    /// if let Ok(stdf) = STDF::from_fname(&fname, verbose) {
    ///     let df: DataFrame = (&stdf.test_data).into();
    ///     let df_fmti: DataFrame = (&stdf.test_data.test_information).into();
    ///     println!("{df:#?}");
    ///     println!("{df_fmti}");
    ///     }
    ///
    /// ```
    /// # Error
    /// If for some reason the file cannot be parsed, returns an `std::io::Error`
    pub fn from_fname(fname: &str, verbose: bool) -> std::io::Result<Self> {
        let test_info = FullTestInformation::from_fname(fname, verbose)?;
        let mut test_data = TestData::new(test_info);
        let mut wirs = Vec::new();
        let mut wrrs = Vec::new();
        let mut soft_bins = HashMap::new();
        let mut hard_bins = HashMap::new();
        let mut pins = HashMap::new();
        let records = Records::new(&fname)?;

        let mut opt_mir: Option<MIR> = None;
        let mut opt_mrr: Option<MRR> = None;
        let mut opt_sdr: Option<SDR> = None;
        for record in records {
            if let Some(resolved) = record.resolve() {
                match resolved {
                    Record::MIR(mir) => {
                        opt_mir = Some(mir);
                    }
                    Record::MRR(mrr) => {
                        opt_mrr = Some(mrr);
                    }
                    Record::SDR(sdr) => {
                        opt_sdr = Some(sdr);
                    }
                    Record::SBR(sbr) => {
                        soft_bins.insert(sbr.sbin_num, sbr);
                    }
                    Record::HBR(hbr) => {
                        hard_bins.insert(hbr.hbin_num, hbr);
                    }
                    Record::PMR(pmr) => {
                        pins.insert(pmr.pmr_indx, pmr);
                    }
                    Record::WIR(wir) => {
                        test_data.new_wafer(&wir);
                        wirs.push(wir);
                    }
                    Record::WRR(wrr) => {
                        test_data.close_wafer();
                        wrrs.push(wrr);
                    }
                    Record::PIR(ref pir) => {
                        test_data.new_part(pir);
                    }
                    Record::PTR(ref ptr) => {
                        test_data.add_data_ptr(ptr);
                    }
                    Record::FTR(ref ftr) => {
                        test_data.add_data_ftr(ftr);
                    }
                    Record::MPR(ref mpr) => {
                        test_data.add_data_mpr(mpr);
                    }
                    Record::PRR(ref prr) => {
                        test_data.finish_part(prr);
                    }
                    _ => {}
                }
            }
        }
        test_data.normalize_multipin_results();
        if let (Some(mir), Some(mrr), Some(site_information)) = (opt_mir, opt_mrr, opt_sdr) {
            let master_information = MasterInformation::new(mir, mrr);
            let wafer_information = wirs
                .into_iter()
                .zip(wrrs.into_iter())
                .map(|(wir, wrr)| WaferInformation::new(wir, wrr))
                .collect();
            Ok(Self {
                master_information,
                wafer_information,
                site_information,
                soft_bins,
                hard_bins,
                pins,
                test_data,
            })
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!("Failed to parse {fname}! MIR or MRR or SDR missing."),
            ))
        }
    }

    /// Convert the HashMap `soft_bins` into a `DataFrame` format
    pub fn soft_bins_to_df(&self) -> DataFrame {
        let mut head_nums: Vec<u8> = Vec::new();
        let mut site_nums: Vec<u8> = Vec::new();
        let mut sbin_nums: Vec<u16> = Vec::new();
        let mut sbin_cnts: Vec<u32> = Vec::new();
        let mut sbin_pfs: Vec<String> = Vec::new();
        let mut sbin_nams: Vec<String> = Vec::new();

        for sbin in self.soft_bins.values() {
            head_nums.push(sbin.head_num);
            site_nums.push(sbin.site_num);
            sbin_nums.push(sbin.sbin_num);
            sbin_cnts.push(sbin.sbin_cnt);
            sbin_pfs.push(sbin.sbin_pf.to_string());
            sbin_nams.push(sbin.sbin_nam.clone());
        }
        let mut columns = Vec::new();
        columns.push(Column::new("sbin_num".into(), sbin_nums));
        columns.push(Column::new("head_num".into(), head_nums));
        columns.push(Column::new("site_num".into(), site_nums));
        columns.push(Column::new("sbin_cnt".into(), sbin_cnts));
        columns.push(Column::new("sbin_pf".into(), sbin_pfs));
        columns.push(Column::new("sbin_nam".into(), sbin_nams));

        DataFrame::new(columns).unwrap()
    }

    /// Convert the HashMap `soft_bins` into a `DataFrame` format
    pub fn hard_bins_to_df(&self) -> DataFrame {
        let mut head_nums: Vec<u8> = Vec::new();
        let mut site_nums: Vec<u8> = Vec::new();
        let mut hbin_nums: Vec<u16> = Vec::new();
        let mut hbin_cnts: Vec<u32> = Vec::new();
        let mut hbin_pfs: Vec<String> = Vec::new();
        let mut hbin_nams: Vec<String> = Vec::new();

        for hbin in self.hard_bins.values() {
            head_nums.push(hbin.head_num);
            site_nums.push(hbin.site_num);
            hbin_nums.push(hbin.hbin_num);
            hbin_cnts.push(hbin.hbin_cnt);
            hbin_pfs.push(hbin.hbin_pf.to_string());
            hbin_nams.push(hbin.hbin_nam.clone());
        }
        let mut columns = Vec::new();
        columns.push(Column::new("hbin_num".into(), hbin_nums));
        columns.push(Column::new("head_num".into(), head_nums));
        columns.push(Column::new("site_num".into(), site_nums));
        columns.push(Column::new("hbin_cnt".into(), hbin_cnts));
        columns.push(Column::new("hbin_pf".into(), hbin_pfs));
        columns.push(Column::new("hbin_nam".into(), hbin_nams));

        DataFrame::new(columns).unwrap()
    }

    /// Convert the HashMap `pin_mapping` into a `DataFrame` format
    pub fn pin_mapping_to_df(&self) -> DataFrame {
        let mut pmr_indxs: Vec<u16> = Vec::new();
        let mut chan_typs: Vec<u16> = Vec::new();
        let mut chan_nams: Vec<String> = Vec::new();
        let mut phy_nams: Vec<String> = Vec::new();
        let mut log_nams: Vec<String> = Vec::new();
        let mut head_nums: Vec<u8> = Vec::new();
        let mut site_nums: Vec<u8> = Vec::new();

        for pmr in self.pins.values() {
            pmr_indxs.push(pmr.pmr_indx);
            chan_typs.push(pmr.chan_typ);
            chan_nams.push(pmr.chan_nam.clone());
            phy_nams.push(pmr.phy_nam.clone());
            log_nams.push(pmr.log_nam.clone());
            head_nums.push(pmr.head_num);
            site_nums.push(pmr.site_num);
        }
        let mut columns = Vec::new();
        columns.push(Column::new("pmr_indx".into(), pmr_indxs));
        columns.push(Column::new("chan_typ".into(), chan_typs));
        columns.push(Column::new("chan_nam".into(), chan_nams));
        columns.push(Column::new("phy_nam".into(), phy_nams));
        columns.push(Column::new("log_nam".into(), log_nams));
        columns.push(Column::new("head_num".into(), head_nums));
        columns.push(Column::new("site_num".into(), site_nums));

        DataFrame::new(columns).unwrap()
    }
}
