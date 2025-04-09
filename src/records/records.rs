use super::Records;
use crate::records::RawRecord;
use crate::util::*;
use pyo3::prelude::IntoPyObject;
use std::io;

/// Master Information Record
#[derive(Debug, IntoPyObject)]
#[allow(dead_code)]
pub struct MIR {
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
}

impl From<&RawRecord> for MIR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let setup_t = U4(contents, &mut offset);
        let start_t = U4(contents, &mut offset);
        let stat_num = U1(contents, &mut offset);
        let mode_cod = C1(contents, &mut offset);
        let rtst_cod = C1(contents, &mut offset);
        let prot_cod = C1(contents, &mut offset);
        let burn_tim = U2(contents, &mut offset);
        let cmod_cod = C1(contents, &mut offset);
        let lot_id = Cn(contents, &mut offset);
        let part_typ = Cn(contents, &mut offset);
        let node_nam = Cn(contents, &mut offset);
        let tstr_typ = Cn(contents, &mut offset);
        let job_nam = Cn(contents, &mut offset);
        let job_rev = Cn(contents, &mut offset);
        let sblot_id = Cn(contents, &mut offset);
        let oper_nam = Cn(contents, &mut offset);
        let exec_typ = Cn(contents, &mut offset);
        let exec_ver = Cn(contents, &mut offset);
        let test_cod = Cn(contents, &mut offset);
        let tst_temp = Cn(contents, &mut offset);
        let user_txt = Cn(contents, &mut offset);
        let aux_file = Cn(contents, &mut offset);
        let pkg_typ = Cn(contents, &mut offset);
        let famly_id = Cn(contents, &mut offset);
        let date_cod = Cn(contents, &mut offset);
        let facil_id = Cn(contents, &mut offset);
        let floor_id = Cn(contents, &mut offset);
        let proc_id = Cn(contents, &mut offset);
        let oper_frq = Cn(contents, &mut offset);
        let spec_nam = Cn(contents, &mut offset);
        let spec_ver = Cn(contents, &mut offset);
        let flow_id = Cn(contents, &mut offset);
        let setup_id = Cn(contents, &mut offset);
        let dsgn_rev = Cn(contents, &mut offset);
        let eng_id = Cn(contents, &mut offset);
        let rom_cod = Cn(contents, &mut offset);
        let serl_num = Cn(contents, &mut offset);
        let supr_nam = Cn(contents, &mut offset);

        Self {
            setup_t,
            start_t,
            stat_num,
            mode_cod,
            rtst_cod,
            prot_cod,
            burn_tim,
            cmod_cod,
            lot_id,
            part_typ,
            node_nam,
            tstr_typ,
            job_nam,
            job_rev,
            sblot_id,
            oper_nam,
            exec_typ,
            exec_ver,
            test_cod,
            tst_temp,
            user_txt,
            aux_file,
            pkg_typ,
            famly_id,
            date_cod,
            facil_id,
            floor_id,
            proc_id,
            oper_frq,
            spec_nam,
            spec_ver,
            flow_id,
            setup_id,
            dsgn_rev,
            eng_id,
            rom_cod,
            serl_num,
            supr_nam,
        }
    }
}

impl MIR {
    /// Get the MIR from a file at `fname`
    ///
    /// Opens the file from scratch and grabs the MIR rather than re-use some open file handle.
    /// The MIR is always right near the beginning of the file, so this is not very expensive
    ///
    /// # Error
    /// If for some reason the file cannot be parsed, returns an `std::io::Error`
    ///
    /// If for some reason an MIR cannot be found in the file, returns a
    /// `std::io::ErrorKind::UnexpectedEof`
    pub fn from_fname(fname: &str) -> std::io::Result<Self> {
        let records = Records::new(&fname)?;

        for record in records {
            if let Some(resolved) = record.resolve() {
                if let Record::MIR(mir) = resolved {
                    return Ok(mir);
                }
            }
        }
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Failed to find MIR in file",
        ))
    }
}

/// Site Description Record
#[derive(Debug, Clone, IntoPyObject)]
#[allow(dead_code)]
pub struct SDR {
    pub head_num: u8,
    pub site_grp: u8,
    pub site_cnt: u8,
    pub site_num: Vec<u8>,
    pub hand_typ: String,
    pub hand_id: String,
    pub card_typ: String,
    pub card_id: String,
    pub load_typ: String,
    pub load_id: String,
    pub dib_typ: String,
    pub dib_id: String,
    pub cabl_typ: String,
    pub cabl_id: String,
    pub cont_typ: String,
    pub cont_id: String,
    pub lasr_typ: String,
    pub lasr_id: String,
    pub extr_typ: String,
    pub extr_i: String,
}

impl From<&RawRecord> for SDR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_grp = U1(contents, &mut offset);
        let site_cnt = U1(contents, &mut offset);
        let site_num = kxU1(contents, site_cnt as usize, &mut offset);
        let hand_typ = Cn(contents, &mut offset);
        let hand_id = Cn(contents, &mut offset);
        let card_typ = Cn(contents, &mut offset);
        let card_id = Cn(contents, &mut offset);
        let load_typ = Cn(contents, &mut offset);
        let load_id = Cn(contents, &mut offset);
        let dib_typ = Cn(contents, &mut offset);
        let dib_id = Cn(contents, &mut offset);
        let cabl_typ = Cn(contents, &mut offset);
        let cabl_id = Cn(contents, &mut offset);
        let cont_typ = Cn(contents, &mut offset);
        let cont_id = Cn(contents, &mut offset);
        let lasr_typ = Cn(contents, &mut offset);
        let lasr_id = Cn(contents, &mut offset);
        let extr_typ = Cn(contents, &mut offset);
        let extr_i = Cn(contents, &mut offset);

        Self {
            head_num,
            site_grp,
            site_cnt,
            site_num,
            hand_typ,
            hand_id,
            card_typ,
            card_id,
            load_typ,
            load_id,
            dib_typ,
            dib_id,
            cabl_typ,
            cabl_id,
            cont_typ,
            cont_id,
            lasr_typ,
            lasr_id,
            extr_typ,
            extr_i,
        }
    }
}

/// Test Synopsis Record
#[derive(Debug, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct TSR {
    pub head_num: u8,
    pub site_num: u8,
    pub test_typ: char,
    pub test_num: u32,
    pub exec_cnt: u32,
    pub fail_cnt: u32,
    pub alrm_cnt: u32,
    pub test_nam: String,
    pub seq_name: String,
    pub test_lbl: String,
    pub opt_flag: u8,
    pub test_tim: f32,
    pub test_min: f32,
    pub test_max: f32,
    pub tst_sums: f32,
    pub tst_sqrs: f32,
}

impl From<&RawRecord> for TSR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let test_typ: char = C1(contents, &mut offset);
        let test_num = U4(contents, &mut offset);
        let exec_cnt = U4(contents, &mut offset);
        let fail_cnt = U4(contents, &mut offset);
        let alrm_cnt = U4(contents, &mut offset);
        let test_nam = Cn(contents, &mut offset);
        let seq_name = Cn(contents, &mut offset);
        let test_lbl = Cn(contents, &mut offset);
        let opt_flag = U1(contents, &mut offset);
        let test_tim = R4(contents, &mut offset);
        let test_min = R4(contents, &mut offset);
        let test_max = R4(contents, &mut offset);
        let tst_sums = R4(contents, &mut offset);
        let tst_sqrs = R4(contents, &mut offset);

        Self {
            head_num,
            site_num,
            test_typ,
            test_num,
            exec_cnt,
            fail_cnt,
            alrm_cnt,
            test_nam,
            seq_name,
            test_lbl,
            opt_flag,
            test_tim,
            test_min,
            test_max,
            tst_sums,
            tst_sqrs,
        }
    }
}

/// Software Bin Record
#[derive(Debug, Clone, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct SBR {
    pub head_num: u8,
    pub site_num: u8,
    pub sbin_num: u16,
    pub sbin_cnt: u32,
    pub sbin_pf: char,
    pub sbin_nam: String,
}

impl From<&RawRecord> for SBR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let sbin_num = U2(contents, &mut offset);
        let sbin_cnt = U4(contents, &mut offset);
        let sbin_pf = C1(contents, &mut offset);
        let sbin_nam = Cn(contents, &mut offset);

        Self {
            head_num,
            site_num,
            sbin_num,
            sbin_cnt,
            sbin_pf,
            sbin_nam,
        }
    }
}

/// Wafer Information Record
#[derive(Debug, Clone, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct WIR {
    pub head_num: u8,
    pub site_grp: u8,
    pub start_t: u32,
    pub wafer_id: String,
}

impl From<&RawRecord> for WIR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_grp = U1(contents, &mut offset);
        let start_t = U4(contents, &mut offset);
        let wafer_id = Cn(contents, &mut offset);

        Self {
            head_num,
            site_grp,
            start_t,
            wafer_id,
        }
    }
}

/// Wafer Results Record
#[derive(Debug, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct WRR {
    pub head_num: u8,
    pub site_grp: u8,
    pub finish_t: u32,
    pub part_cnt: u32,
    pub rtst_cnt: u32,
    pub abrt_cnt: u32,
    pub good_cnt: u32,
    pub func_cnt: u32,
    pub wafer_id: String,
    pub fabwf_id: String,
    pub frame_id: String,
    pub mask_id: String,
    pub usr_desc: String,
    pub exc_desc: String,
}

impl From<&RawRecord> for WRR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_grp = U1(contents, &mut offset);
        let finish_t = U4(contents, &mut offset);
        let part_cnt = U4(contents, &mut offset);
        let rtst_cnt = U4(contents, &mut offset);
        let abrt_cnt = U4(contents, &mut offset);
        let good_cnt = U4(contents, &mut offset);
        let func_cnt = U4(contents, &mut offset);
        let wafer_id = Cn(contents, &mut offset);
        let fabwf_id = Cn(contents, &mut offset);
        let frame_id = Cn(contents, &mut offset);
        let mask_id = Cn(contents, &mut offset);
        let usr_desc = Cn(contents, &mut offset);
        let exc_desc = Cn(contents, &mut offset);

        Self {
            head_num,
            site_grp,
            finish_t,
            part_cnt,
            rtst_cnt,
            abrt_cnt,
            good_cnt,
            func_cnt,
            wafer_id,
            fabwf_id,
            frame_id,
            mask_id,
            usr_desc,
            exc_desc,
        }
    }
}

/// Hardware Bin Record
#[derive(Debug, Clone, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct HBR {
    pub head_num: u8,
    pub site_num: u8,
    pub hbin_num: u16,
    pub hbin_cnt: u32,
    pub hbin_pf: char,
    pub hbin_nam: String,
}

impl From<&RawRecord> for HBR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let hbin_num = U2(contents, &mut offset);
        let hbin_cnt = U4(contents, &mut offset);
        let hbin_pf = C1(contents, &mut offset);
        let hbin_nam = Cn(contents, &mut offset);

        Self {
            head_num,
            site_num,
            hbin_num,
            hbin_cnt,
            hbin_pf,
            hbin_nam,
        }
    }
}

/// Part Count Record
#[derive(Debug, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct PCR {
    pub head_num: u8,
    pub site_num: u8,
    pub part_cnt: u32,
    pub rtst_cnt: u32,
    pub abrt_cnt: u32,
    pub good_cnt: u32,
    pub func_cnt: u32,
}

impl From<&RawRecord> for PCR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let part_cnt = U4(contents, &mut offset);
        let rtst_cnt = U4(contents, &mut offset);
        let abrt_cnt = U4(contents, &mut offset);
        let good_cnt = U4(contents, &mut offset);
        let func_cnt = U4(contents, &mut offset);

        Self {
            head_num,
            site_num,
            part_cnt,
            rtst_cnt,
            abrt_cnt,
            good_cnt,
            func_cnt,
        }
    }
}

/// Part Information Record
#[derive(Debug, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct PIR {
    pub head_num: u8,
    pub site_num: u8,
}

impl From<&RawRecord> for PIR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let head_num = contents[0];
        let site_num = contents[1];

        Self { head_num, site_num }
    }
}

/// Part Results Record
#[derive(Debug, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct PRR {
    pub head_num: u8,
    pub site_num: u8,
    pub part_flg: u8,
    pub num_test: u16,
    pub hard_bin: u16,
    pub soft_bin: u16,
    pub x_coord: i16,
    pub y_coord: i16,
    pub test_t: u32,
    pub part_id: String,
    pub part_txt: String,
    pub part_fix: Vec<u8>,
}

impl From<&RawRecord> for PRR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let part_flg = U1(contents, &mut offset);
        let num_test = U2(contents, &mut offset);
        let hard_bin = U2(contents, &mut offset);
        let soft_bin = U2(contents, &mut offset);
        let x_coord = I2(contents, &mut offset);
        let y_coord = I2(contents, &mut offset);
        let test_t = U4(contents, &mut offset);
        let part_id = Cn(contents, &mut offset);
        let part_txt = Cn(contents, &mut offset);
        let part_fix = Bn(contents, &mut offset);

        Self {
            head_num,
            site_num,
            part_flg,
            num_test,
            hard_bin,
            soft_bin,
            x_coord,
            y_coord,
            test_t,
            part_id,
            part_txt,
            part_fix,
        }
    }
}

/// Master Results Record
#[derive(Debug, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct MRR {
    pub finish_t: u32,
    pub disp_cod: char,
    pub usr_desc: String,
    pub exc_desc: String,
}

impl From<&RawRecord> for MRR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let finish_t = U4(contents, &mut offset);
        let disp_cod = C1(contents, &mut offset);
        let usr_desc = Cn(contents, &mut offset);
        let exc_desc = Cn(contents, &mut offset);

        Self {
            finish_t,
            disp_cod,
            usr_desc,
            exc_desc,
        }
    }
}

/// Parametric Test Record
#[derive(Debug, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct PTR {
    pub test_num: u32,
    pub head_num: u8,
    pub site_num: u8,
    pub test_flg: u8,
    pub parm_flg: u8,
    pub result: f32,
    pub test_txt: String,
    pub alarm_id: String,
    pub opt_flag: u8,
    pub res_scal: i8,
    pub llm_scal: i8,
    pub hlm_scal: i8,
    pub lo_limit: f32,
    pub hi_limit: f32,
    pub units: String,
    pub c_resfmt: String,
    pub c_llmfmt: String,
    pub c_hlmfmt: String,
    pub lo_spec: f32,
    pub hi_spec: f32,
}

impl From<&RawRecord> for PTR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let test_num = U4(contents, &mut offset);
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let test_flg = U1(contents, &mut offset);
        let parm_flg = U1(contents, &mut offset);
        let result = R4(contents, &mut offset);
        let test_txt = Cn(contents, &mut offset);
        let alarm_id = Cn(contents, &mut offset);
        let opt_flag;
        let res_scal;
        let llm_scal;
        let hlm_scal;
        let lo_limit;
        let hi_limit;
        let units;
        let c_resfmt;
        let c_llmfmt;
        let c_hlmfmt;
        let lo_spec;
        let hi_spec;
        if offset < record.contents.len() {
            opt_flag = U1(contents, &mut offset);
            res_scal = I1(contents, &mut offset);
            llm_scal = I1(contents, &mut offset);
            hlm_scal = I1(contents, &mut offset);
            lo_limit = R4(contents, &mut offset);
            hi_limit = R4(contents, &mut offset);
            units = Cn(contents, &mut offset);
            c_resfmt = Cn(contents, &mut offset);
            c_llmfmt = Cn(contents, &mut offset);
            c_hlmfmt = Cn(contents, &mut offset);
            lo_spec = R4(contents, &mut offset);
            hi_spec = R4(contents, &mut offset);
        } else {
            opt_flag = 0;
            res_scal = 0;
            llm_scal = 0;
            hlm_scal = 0;
            lo_limit = 0.;
            hi_limit = 0.;
            units = "".to_string();
            c_resfmt = "".to_string();
            c_llmfmt = "".to_string();
            c_hlmfmt = "".to_string();
            lo_spec = 0.;
            hi_spec = 0.;
        }

        Self {
            test_num,
            head_num,
            site_num,
            test_flg,
            parm_flg,
            result,
            test_txt,
            alarm_id,
            opt_flag,
            res_scal,
            llm_scal,
            hlm_scal,
            lo_limit,
            hi_limit,
            units,
            c_resfmt,
            c_llmfmt,
            c_hlmfmt,
            lo_spec,
            hi_spec,
        }
    }
}

impl PTR {
    pub fn pass(&self) -> bool {
        (self.test_flg >> 6) & 0b11 == 0
    }
}

/// Functional Test Record
#[derive(Debug, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct FTR {
    pub test_num: u32,
    pub head_num: u8,
    pub site_num: u8,
    pub test_flg: u8,
    pub opt_flag: u8,
    pub cycl_cnt: u32,
    pub rel_vadr: u32,
    pub rept_cnt: u32,
    pub num_fail: u32,
    pub xfail_ad: i32,
    pub yfail_ad: i32,
    pub vect_off: i16,
    pub rtn_icnt: u16,      // j
    pub pgm_icnt: u16,      // k
    pub rtn_indx: Vec<u16>, // rtn_icnt
    pub rtn_stat: Vec<u8>,  // rtn_icnt, nibbles
    pub pgm_indx: Vec<u16>, // pgm_icnt
    pub pgm_stat: Vec<u8>,  // pgm_icnt, nibbles
    pub fail_pin: Vec<u8>,  // Dn type (first 2 bytes length)
    pub vect_nam: String,
    pub time_set: String,
    pub op_code: String,
    pub test_txt: String,
    pub alarm_id: String,
    pub prog_txt: String,
    pub rslt_txt: String,
    pub patg_num: u8,
    pub spin_map: Vec<u8>, // Dn type (first 2 bytes length)
}

impl From<&RawRecord> for FTR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;
        let test_num = U4(contents, &mut offset);
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let test_flg = U1(contents, &mut offset);
        let opt_flag = U1(contents, &mut offset);
        let cycl_cnt = U4(contents, &mut offset);
        let rel_vadr = U4(contents, &mut offset);
        let rept_cnt = U4(contents, &mut offset);
        let num_fail = U4(contents, &mut offset);
        let xfail_ad = I4(contents, &mut offset);
        let yfail_ad = I4(contents, &mut offset);
        let vect_off = I2(contents, &mut offset);
        let rtn_icnt = U2(contents, &mut offset);
        let pgm_icnt = U2(contents, &mut offset);
        let rtn_indx = kxU2(contents, rtn_icnt.into(), &mut offset);
        let rtn_stat = kxN1(contents, rtn_icnt.into(), &mut offset);
        let pgm_indx = kxU2(contents, pgm_icnt.into(), &mut offset);
        let pgm_stat = kxN1(contents, pgm_icnt.into(), &mut offset);
        let fail_pin = Dn(contents, &mut offset);

        let vect_nam = Cn(contents, &mut offset);
        let time_set = Cn(contents, &mut offset);
        let op_code = Cn(contents, &mut offset);
        let test_txt = Cn(contents, &mut offset);
        let alarm_id = Cn(contents, &mut offset);
        let prog_txt = Cn(contents, &mut offset);
        let rslt_txt = Cn(contents, &mut offset);
        let patg_num = U1(contents, &mut offset);
        let spin_map = Dn(contents, &mut offset);

        Self {
            test_num,
            head_num,
            site_num,
            test_flg,
            opt_flag,
            cycl_cnt,
            rel_vadr,
            rept_cnt,
            num_fail,
            xfail_ad,
            yfail_ad,
            vect_off,
            rtn_icnt,
            pgm_icnt,
            rtn_indx,
            rtn_stat,
            pgm_indx,
            pgm_stat,
            fail_pin,
            vect_nam,
            time_set,
            op_code,
            test_txt,
            alarm_id,
            prog_txt,
            rslt_txt,
            patg_num,
            spin_map,
        }
    }
}

impl FTR {
    pub fn get_passfail(&self) -> bool {
        let test_flg = self.test_flg;
        // don't bother checking the other flags
        test_flg & 0x80 == 0
    }
}

/// Multiple-Result Parametric Record
#[derive(Debug, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct MPR {
    pub test_num: u32,
    pub head_num: u8,
    pub site_num: u8,
    pub test_flg: u8,
    pub parm_flg: u8,
    pub rtn_icnt: u16,      // j
    pub rslt_cnt: u16,      // k
    pub rtn_stat: Vec<u8>,  // jxN*1
    pub rtn_rslt: Vec<f32>, // kxR*4
    pub test_txt: String,
    pub alarm_id: String,
    pub opt_flag: u8,
    pub res_scal: i8,
    pub llm_scal: i8,
    pub hlm_scal: i8,
    pub lo_limit: f32,
    pub hi_limit: f32,
    pub start_in: f32,
    pub incr_in: f32,
    pub rtn_indx: Vec<u16>, // jxU*2
    pub units: String,
    pub units_in: String,
    pub c_resfmt: String,
    pub c_llmfmt: String,
    pub c_hlmfmt: String,
    pub lo_spec: f32,
    pub hi_spec: f32,
}

impl From<&RawRecord> for MPR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;

        let test_num = U4(contents, &mut offset);
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);
        let test_flg = U1(contents, &mut offset);
        let parm_flg = U1(contents, &mut offset);
        let rtn_icnt = U2(contents, &mut offset);
        let rslt_cnt = U2(contents, &mut offset);
        let rtn_stat = kxN1(contents, rtn_icnt.into(), &mut offset);
        let rtn_rslt = kxR4(contents, rslt_cnt.into(), &mut offset);
        let test_txt = Cn(contents, &mut offset);
        let alarm_id = Cn(contents, &mut offset);
        let opt_flag = U1(contents, &mut offset);
        let res_scal = I1(contents, &mut offset);
        let llm_scal = I1(contents, &mut offset);
        let hlm_scal = I1(contents, &mut offset);
        let lo_limit = R4(contents, &mut offset);
        let hi_limit = R4(contents, &mut offset);
        let start_in = R4(contents, &mut offset);
        let incr_in = R4(contents, &mut offset);
        let rtn_indx = kxU2(contents, rtn_icnt.into(), &mut offset);
        let units = Cn(contents, &mut offset);
        let units_in = Cn(contents, &mut offset);
        let c_resfmt = Cn(contents, &mut offset);
        let c_llmfmt = Cn(contents, &mut offset);
        let c_hlmfmt = Cn(contents, &mut offset);
        let lo_spec = R4(contents, &mut offset);
        let hi_spec = R4(contents, &mut offset);

        Self {
            test_num,
            head_num,
            site_num,
            test_flg,
            parm_flg,
            rtn_icnt,
            rslt_cnt,
            rtn_stat,
            rtn_rslt,
            test_txt,
            alarm_id,
            opt_flag,
            res_scal,
            llm_scal,
            hlm_scal,
            lo_limit,
            hi_limit,
            start_in,
            incr_in,
            rtn_indx,
            units,
            units_in,
            c_resfmt,
            c_llmfmt,
            c_hlmfmt,
            lo_spec,
            hi_spec,
        }
    }
}

/// Pin Map Record
#[derive(Debug, IntoPyObject)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct PMR {
    pub pmr_indx: u16,
    pub chan_typ: u16,
    pub chan_nam: String,
    pub phy_nam: String,
    pub log_nam: String,
    pub head_num: u8,
    pub site_num: u8,
}

impl From<&RawRecord> for PMR {
    fn from(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let mut offset: usize = 0;

        let pmr_indx = U2(contents, &mut offset);
        let chan_typ = U2(contents, &mut offset);
        let chan_nam = Cn(contents, &mut offset);
        let phy_nam = Cn(contents, &mut offset);
        let log_nam = Cn(contents, &mut offset);
        let head_num = U1(contents, &mut offset);
        let site_num = U1(contents, &mut offset);

        Self {
            pmr_indx,
            chan_typ,
            chan_nam,
            phy_nam,
            log_nam,
            head_num,
            site_num,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct NotImplementedRecord {}

/// An enum of all the concrete record types
#[derive(Debug)]
pub enum Record {
    FAR(NotImplementedRecord),
    ATR(NotImplementedRecord),
    MIR(MIR),
    MRR(MRR),
    PCR(PCR),
    HBR(HBR),
    SBR(SBR),
    PMR(PMR),
    PGR(NotImplementedRecord),
    PLR(NotImplementedRecord),
    RDR(NotImplementedRecord),
    SDR(SDR),
    WIR(WIR),
    WRR(WRR),
    WCR(NotImplementedRecord),
    PIR(PIR),
    PRR(PRR),
    TSR(TSR),
    PTR(PTR),
    MPR(MPR),
    FTR(FTR),
    BPS(NotImplementedRecord),
    EPS(NotImplementedRecord),
    GDR(NotImplementedRecord),
    DTR(NotImplementedRecord),
    InvalidRecord(NotImplementedRecord),
}
