use crate::records::RawRecord;
use crate::util::bn_from_bytes;
use crate::util::cn_from_bytes;

#[derive(Debug)]
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

impl MIR {
    pub fn from_raw_record(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let setup_t = u32::from_le_bytes(contents[..4].try_into().unwrap());
        let start_t = u32::from_le_bytes(contents[4..8].try_into().unwrap());
        let stat_num = contents[8];
        let mode_cod: char = char::from_u32(contents[9] as u32).expect("Invalid MIR.MODE_COD");
        let rtst_cod: char = char::from_u32(contents[10] as u32).expect("Invalid MIR.RTST_COD");
        let prot_cod: char = char::from_u32(contents[11] as u32).expect("Invalid MIR.PROT_COD");
        let burn_tim: u16 = u16::from_le_bytes(contents[12..14].try_into().unwrap());
        let cmod_cod: char = char::from_u32(contents[14] as u32).expect("Invalid MIR.CMOD_COD");

        let mut offset: usize = 15;
        let lot_id = cn_from_bytes(contents, offset).expect("Invalid MIR.LOT_ID");
        offset += lot_id.len() + 1;
        let part_typ = cn_from_bytes(contents, offset).expect("Invalid MIR.PART_TYP");
        offset += part_typ.len() + 1;
        let node_nam = cn_from_bytes(contents, offset).expect("Invalid MIR.NODE_NAM");
        offset += node_nam.len() + 1;
        let tstr_typ = cn_from_bytes(contents, offset).expect("Invalid MIR.NODE_NAM");
        offset += tstr_typ.len() + 1;
        let job_nam = cn_from_bytes(contents, offset).expect("Invalid MIR.job_nam");
        offset += job_nam.len() + 1;
        let job_rev = cn_from_bytes(contents, offset).expect("Invalid MIR.NODE_NAM");
        offset += job_rev.len() + 1;
        let sblot_id = cn_from_bytes(contents, offset).expect("Invalid MIR.NODE_NAM");
        offset += sblot_id.len() + 1;
        let oper_nam = cn_from_bytes(contents, offset).expect("Invalid MIR.NODE_NAM");
        offset += oper_nam.len() + 1;
        let exec_typ = cn_from_bytes(contents, offset).expect("Invalid MIR.NODE_NAM");
        offset += exec_typ.len() + 1;
        let exec_ver = cn_from_bytes(contents, offset).expect("Invalid MIR.EXEC_VER");
        offset += exec_ver.len() + 1;
        let test_cod = cn_from_bytes(contents, offset).expect("Invalid MIR.TEST_COD");
        offset += test_cod.len() + 1;
        let tst_temp = cn_from_bytes(contents, offset).expect("Invalid MIR.TST_TEMP");
        offset += tst_temp.len() + 1;
        let user_txt = cn_from_bytes(contents, offset).expect("Invalid MIR.USER_TXT");
        offset += user_txt.len() + 1;
        let aux_file = cn_from_bytes(contents, offset).expect("Invalid MIR.AUX_FILE");
        offset += aux_file.len() + 1;
        let pkg_typ = cn_from_bytes(contents, offset).expect("Invalid MIR.PKG_TYP");
        offset += pkg_typ.len() + 1;
        let famly_id = cn_from_bytes(contents, offset).expect("Invalid MIR.FAMLY_ID");
        offset += famly_id.len() + 1;
        let date_cod = cn_from_bytes(contents, offset).expect("Invalid MIR.DATE_COD");
        offset += date_cod.len() + 1;
        let facil_id = cn_from_bytes(contents, offset).expect("Invalid MIR.FACIL_ID");
        offset += facil_id.len() + 1;
        let floor_id = cn_from_bytes(contents, offset).expect("Invalid MIR.FLOOR_ID");
        offset += floor_id.len() + 1;
        let proc_id = cn_from_bytes(contents, offset).expect("Invalid MIR.PROC_ID");
        offset += proc_id.len() + 1;
        let oper_frq = cn_from_bytes(contents, offset).expect("Invalid MIR.OPER_FRQ");
        offset += oper_frq.len() + 1;
        let spec_nam = cn_from_bytes(contents, offset).expect("Invalid MIR.SPEC_NAM");
        offset += spec_nam.len() + 1;
        let spec_ver = cn_from_bytes(contents, offset).expect("Invalid MIR.SPEC_VER");
        offset += spec_ver.len() + 1;
        let flow_id = cn_from_bytes(contents, offset).expect("Invalid MIR.FLOW_ID");
        offset += flow_id.len() + 1;
        let setup_id = cn_from_bytes(contents, offset).expect("Invalid MIR.SETUP_ID");
        offset += setup_id.len() + 1;
        let dsgn_rev = cn_from_bytes(contents, offset).expect("Invalid MIR.DSGN_REV");
        offset += dsgn_rev.len() + 1;
        let eng_id = cn_from_bytes(contents, offset).expect("Invalid MIR.ENG_ID");
        offset += eng_id.len() + 1;
        let rom_cod = cn_from_bytes(contents, offset).expect("Invalid MIR.ROM_COD");
        offset += rom_cod.len() + 1;
        let serl_num = cn_from_bytes(contents, offset).expect("Invalid MIR.SERL_NUM");
        offset += serl_num.len() + 1;
        let supr_nam = cn_from_bytes(contents, offset).expect("Invalid MIR.SUPR_NAM");

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

#[derive(Debug)]
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

impl SDR {
    pub fn from_raw_record(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let head_num = contents[0];
        let site_grp = contents[1];
        let site_cnt = contents[2];
        let mut offset = 3 + site_cnt as usize;
        let site_num = contents[3..offset].to_vec();

        let hand_typ = cn_from_bytes(contents, offset).expect("Invalid SDR.HAND_TYP");
        offset += hand_typ.len() + 1;
        let hand_id = cn_from_bytes(contents, offset).expect("Invalid SDR.HAND_ID");
        offset += hand_id.len() + 1;
        let card_typ = cn_from_bytes(contents, offset).expect("Invalid SDR.CARD_TYP");
        offset += card_typ.len() + 1;
        let card_id = cn_from_bytes(contents, offset).expect("Invalid SDR.CARD_ID");
        offset += card_id.len() + 1;
        let load_typ = cn_from_bytes(contents, offset).expect("Invalid SDR.LOAD_TYP");
        offset += load_typ.len() + 1;
        let load_id = cn_from_bytes(contents, offset).expect("Invalid SDR.LOAD_ID");
        offset += load_id.len() + 1;
        let dib_typ = cn_from_bytes(contents, offset).expect("Invalid SDR.DIB_TYP");
        offset += dib_typ.len() + 1;
        let dib_id = cn_from_bytes(contents, offset).expect("Invalid SDR.DIB_ID");
        offset += dib_id.len() + 1;
        let cabl_typ = cn_from_bytes(contents, offset).expect("Invalid SDR.CABL_TYP");
        offset += cabl_typ.len() + 1;
        let cabl_id = cn_from_bytes(contents, offset).expect("Invalid SDR.CABL_ID");
        offset += cabl_id.len() + 1;
        let cont_typ = cn_from_bytes(contents, offset).expect("Invalid SDR.CONT_TYP");
        offset += cont_typ.len() + 1;
        let cont_id = cn_from_bytes(contents, offset).expect("Invalid SDR.CONT_ID");
        offset += cont_id.len() + 1;
        let lasr_typ = cn_from_bytes(contents, offset).expect("Invalid SDR.LASR_TYP");
        offset += lasr_typ.len() + 1;
        let lasr_id = cn_from_bytes(contents, offset).expect("Invalid SDR.LASR_ID");
        offset += lasr_id.len() + 1;
        let extr_typ = cn_from_bytes(contents, offset).expect("Invalid SDR.EXTR_TYP");
        offset += extr_typ.len() + 1;
        let extr_i = cn_from_bytes(contents, offset).expect("Invalid SDR.EXTR_I");

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

#[derive(Debug)]
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

impl TSR {
    pub fn from_raw_record(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let head_num = contents[0];
        let site_num = contents[1];
        let test_typ: char = char::from_u32(contents[2] as u32).expect("Invalid TSR.TEST_TYP");
        let test_num = u32::from_le_bytes(contents[3..7].try_into().unwrap());
        let exec_cnt = u32::from_le_bytes(contents[7..11].try_into().unwrap());
        let fail_cnt = u32::from_le_bytes(contents[11..15].try_into().unwrap());
        let alrm_cnt = u32::from_le_bytes(contents[15..19].try_into().unwrap());
        let mut offset: usize = 19;
        let test_nam = cn_from_bytes(contents, offset).expect("Invalid TSR.TEST_NAM");
        offset += test_nam.len() + 1;
        let seq_name = cn_from_bytes(contents, offset).expect("Invalid TSR.SEQ_NAME");
        offset += seq_name.len() + 1;
        let test_lbl = cn_from_bytes(contents, offset).expect("Invalid TSR.TEST_LBL");
        offset += test_lbl.len() + 1;
        let opt_flag = contents[offset];
        offset += 1;
        let test_tim = f32::from_le_bytes(contents[offset..offset + 4].try_into().unwrap());
        let test_min = f32::from_le_bytes(contents[offset + 4..offset + 8].try_into().unwrap());
        let test_max = f32::from_le_bytes(contents[offset + 8..offset + 12].try_into().unwrap());
        let tst_sums = f32::from_le_bytes(contents[offset + 12..offset + 16].try_into().unwrap());
        let tst_sqrs = f32::from_le_bytes(contents[offset + 16..offset + 20].try_into().unwrap());

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

#[derive(Debug)]
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

impl SBR {
    pub fn from_raw_record(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let head_num = contents[0];
        let site_num = contents[1];
        let sbin_num = u16::from_le_bytes(contents[2..4].try_into().unwrap());
        let sbin_cnt = u32::from_le_bytes(contents[4..8].try_into().unwrap());
        let sbin_pf = char::from_u32(contents[8] as u32).expect("Invalid SBR.SBIN_PF");
        let sbin_nam = cn_from_bytes(contents, 9).expect("Invalid SBR.SBIN_NAM");

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

#[derive(Debug)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct WIR {
    pub head_num: u8,
    pub site_grp: u8,
    pub start_t: u32,
    pub wafer_id: String,
}

impl WIR {
    pub fn from_raw_record(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let head_num = contents[0];
        let site_grp = contents[1];
        let start_t = u32::from_le_bytes(contents[2..6].try_into().unwrap());
        let wafer_id = cn_from_bytes(contents, 6).expect("Invalid WIR.WAFER_ID");

        Self {
            head_num,
            site_grp,
            start_t,
            wafer_id,
        }
    }
}

#[derive(Debug)]
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

impl WRR {
    pub fn from_raw_record(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let head_num = contents[0];
        let site_grp = contents[1];
        let finish_t = u32::from_le_bytes(contents[2..6].try_into().unwrap());
        let part_cnt = u32::from_le_bytes(contents[6..10].try_into().unwrap());
        let rtst_cnt = u32::from_le_bytes(contents[10..14].try_into().unwrap());
        let abrt_cnt = u32::from_le_bytes(contents[14..18].try_into().unwrap());
        let good_cnt = u32::from_le_bytes(contents[18..22].try_into().unwrap());
        let func_cnt = u32::from_le_bytes(contents[22..26].try_into().unwrap());
        let mut offset = 26;
        let wafer_id = cn_from_bytes(contents, offset).expect("Invalid WRR.WAFER_ID");
        offset += 1 + wafer_id.len();
        let fabwf_id = cn_from_bytes(contents, offset).expect("Invalid WRR.FABWF_ID");
        offset += 1 + fabwf_id.len();
        let frame_id = cn_from_bytes(contents, offset).expect("Invalid WRR.FRAME_ID");
        offset += 1 + frame_id.len();
        let mask_id = cn_from_bytes(contents, offset).expect("Invalid WRR.MASK_ID");
        offset += 1 + mask_id.len();
        let usr_desc = cn_from_bytes(contents, offset).expect("Invalid WRR.USR_DESC");
        offset += 1 + usr_desc.len();
        let exc_desc = cn_from_bytes(contents, offset).expect("Invalid WRR.EXC_DESC");

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

#[derive(Debug)]
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

impl HBR {
    pub fn from_raw_record(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let head_num = contents[0];
        let site_num = contents[1];
        let hbin_num = u16::from_le_bytes(contents[2..4].try_into().unwrap());
        let hbin_cnt = u32::from_le_bytes(contents[4..8].try_into().unwrap());
        let hbin_pf = char::from_u32(contents[8] as u32).expect("Invalid HBR.HBIN_PF");
        let hbin_nam = cn_from_bytes(contents, 9).expect("Invalid HBR.HBIN_NAM");

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

#[derive(Debug)]
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

impl PCR {
    pub fn from_raw_record(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let head_num = contents[0];
        let site_num = contents[1];
        let part_cnt = u32::from_le_bytes(contents[2..6].try_into().unwrap());
        let rtst_cnt = u32::from_le_bytes(contents[6..10].try_into().unwrap());
        let abrt_cnt = u32::from_le_bytes(contents[10..14].try_into().unwrap());
        let good_cnt = u32::from_le_bytes(contents[14..18].try_into().unwrap());
        let func_cnt = u32::from_le_bytes(contents[18..22].try_into().unwrap());

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

#[derive(Debug)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct PIR {
    pub head_num: u8,
    pub site_num: u8,
}

impl PIR {
    pub fn from_raw_record(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let head_num = contents[0];
        let site_num = contents[1];

        Self { head_num, site_num }
    }
}

#[derive(Debug)]
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

impl PRR {
    pub fn from_raw_record(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let head_num = contents[0];
        let site_num = contents[1];
        let part_flg = contents[2];
        let num_test = u16::from_le_bytes(contents[3..5].try_into().unwrap());
        let hard_bin = u16::from_le_bytes(contents[5..7].try_into().unwrap());
        let soft_bin = u16::from_le_bytes(contents[7..9].try_into().unwrap());
        let x_coord = i16::from_le_bytes(contents[9..11].try_into().unwrap());
        let y_coord = i16::from_le_bytes(contents[11..13].try_into().unwrap());
        let test_t = u32::from_le_bytes(contents[13..17].try_into().unwrap());
        let part_id = cn_from_bytes(contents, 17).expect("Invalid PRR.PART_ID");
        let mut offset = 18 + part_id.len() as usize;
        let part_txt = cn_from_bytes(contents, offset).expect("Invalid PRR.PART_TXT");
        offset += 1 + part_txt.len() as usize;
        let part_fix = bn_from_bytes(contents, offset);

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

#[derive(Debug)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct MRR {
    pub finish_t: u32,
    pub disp_cod: char,
    pub usr_desc: String,
    pub exc_desc: String,
}

impl MRR {
    pub fn from_raw_record(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let finish_t = u32::from_le_bytes(contents[..4].try_into().unwrap());
        let disp_cod = char::from_u32(contents[4] as u32).expect("Invalid MRR.DISP_COD");
        let usr_desc = cn_from_bytes(contents, 5).expect("Invalid MRR.USR_DESC");
        let offset = 6 + usr_desc.len() as usize;
        let exc_desc = cn_from_bytes(contents, offset).expect("Invalid MRR.EXC_DESC");

        Self {
            finish_t,
            disp_cod,
            usr_desc,
            exc_desc,
        }
    }
}

#[derive(Debug)]
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

impl PTR {
    pub fn from_raw_record(record: &RawRecord) -> Self {
        let contents = &record.contents;
        let test_num = u32::from_le_bytes(contents[..4].try_into().unwrap());
        let head_num = contents[4];
        let site_num = contents[5];
        let test_flg = contents[6];
        let parm_flg = contents[7];
        let result = f32::from_le_bytes(contents[8..12].try_into().unwrap());
        let mut offset = 12;
        let test_txt = cn_from_bytes(contents, offset).expect("Invalid PTR.TEST_TXT");
        offset += 1 + test_txt.len();
        let alarm_id = cn_from_bytes(contents, offset).expect("Invalid PTR.ALARM_ID");
        offset += 1 + alarm_id.len();
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
            opt_flag = contents[offset];
            offset += 1;
            res_scal = i8::from_le_bytes(contents[offset..offset + 1].try_into().unwrap());
            offset += 1;
            llm_scal = i8::from_le_bytes(contents[offset..offset + 1].try_into().unwrap());
            offset += 1;
            hlm_scal = i8::from_le_bytes(contents[offset..offset + 1].try_into().unwrap());
            offset += 1;
            lo_limit = f32::from_le_bytes(contents[offset..offset + 4].try_into().unwrap());
            offset += 4;
            hi_limit = f32::from_le_bytes(contents[offset..offset + 4].try_into().unwrap());
            offset += 4;
            units = cn_from_bytes(contents, offset).expect("Invalid PTR.UNITS");
            offset += 1 + units.len();
            c_resfmt = cn_from_bytes(contents, offset).expect("Invalid PTR.C_RESFMT");
            offset += 1 + c_resfmt.len();
            c_llmfmt = cn_from_bytes(contents, offset).expect("Invalid PTR.C_LLMFMT");
            offset += 1 + c_llmfmt.len();
            c_hlmfmt = cn_from_bytes(contents, offset).expect("Invalid PTR.C_HLMFMT");
            offset += 1 + c_hlmfmt.len();
            lo_spec = f32::from_le_bytes(contents[offset..offset + 4].try_into().unwrap());
            offset += 4;
            hi_spec = f32::from_le_bytes(contents[offset..offset + 4].try_into().unwrap());
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

#[derive(Debug)]
#[allow(dead_code)]
pub struct NotImplementedRecord {}

#[derive(Debug)]
pub enum Record {
    FAR(NotImplementedRecord),
    ATR(NotImplementedRecord),
    MIR(MIR),
    MRR(MRR),
    PCR(PCR),
    HBR(HBR),
    SBR(SBR),
    PMR(NotImplementedRecord),
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
    MPR(NotImplementedRecord),
    FTR(NotImplementedRecord),
    BPS(NotImplementedRecord),
    EPS(NotImplementedRecord),
    GDR(NotImplementedRecord),
    DTR(NotImplementedRecord),
    InvalidRecord(NotImplementedRecord),
}
