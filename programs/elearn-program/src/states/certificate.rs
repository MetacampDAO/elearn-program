use anchor_lang::prelude::*;

pub const LATEST_CERTIFICATE_VERSION: u16 = 1;

pub const CERTIFICATE_PDA_SEED: &[u8] = b"certificate-seed";

pub const MAX_STUDENT_NAME_LEN: usize = 64;
pub const MAX_STUDENT_GRADE_LEN: usize = 32;

pub const MAX_COURSE_NAME_LEN: usize = 64;

pub const MAX_SCHOOL_NAME_LEN: usize = 64;
pub const MAX_SCHOOL_URI_LEN: usize = 200;

pub const MAX_ISSUER_NAME_LEN: usize = 64;
pub const MAX_ISSUER_ROLE_LEN: usize = 32;
pub const MAX_ISSUER_URI_LEN: usize = 200;



pub const MAX_CERTIFICATE_LEN: usize = 2 // version
  + 32 // batch pda
  + 32  // manager key
  + 32  // student key
  + 8   // start date
  + 8   // end date
  + 8   // completion date
  + 8   // certificate number
  + 1   // certificate bump
  + MAX_STUDENT_NAME_LEN
  + MAX_STUDENT_GRADE_LEN
  + MAX_COURSE_NAME_LEN
  + MAX_SCHOOL_NAME_LEN
  + MAX_SCHOOL_URI_LEN
  + MAX_ISSUER_NAME_LEN
  + MAX_ISSUER_ROLE_LEN
  + MAX_ISSUER_URI_LEN;

  #[account]
pub struct Certificate {
  pub version: u16,
  pub batch_pda: Pubkey,
  pub manager_key: Pubkey,
  pub student_key: Pubkey,
  pub complete_date: u64,
  pub certificate_num: u64,
  pub certificate_bump: u8,
  pub student_name: String,
  pub student_grade: String,
  pub course_name: String,
  pub school_name: String,
  pub school_uri: String,
  pub issuer_name: String,
  pub issuer_role:String,
  pub issuer_uri: String,
}

#[account]
pub struct CertificateV1 {
  pub version: u16,
  pub batch_pda: Pubkey,
  pub manager_key: Pubkey,
  pub student_key: Pubkey,
  pub start_date: u64,
  pub end_date: u64,
  pub complete_date: u64,
  pub certificate_num: u64,
  pub certificate_bump: u8,
  pub student_name: String,
  pub student_grade: String,
  pub course_name: String,
  pub school_name: String,
  pub school_uri: String,
  pub issuer_name: String,
  pub issuer_role:String,
  pub issuer_uri: String,
}

impl CertificateV1 {
  pub fn initialize(&mut self,
    batch_pda: Pubkey,
    manager_key: Pubkey,
    student_key: Pubkey,
    start_date: u64,
    end_date: u64,
    complete_date: u64,
    certificate_num: u64,
    certificate_bump: u8,
    student_name: String,
    student_grade: String,
    course_name: String,
    school_name: String,
    school_uri: String,
    issuer_name: String,
    issuer_role:String,
    issuer_uri: String,
  ) {
    self.version = LATEST_CERTIFICATE_VERSION;
    self.batch_pda = batch_pda;
    self.manager_key = manager_key;
    self.student_key = student_key;
    self.start_date = start_date;
    self.end_date = end_date;
    self.complete_date = complete_date;
    self.certificate_num = certificate_num;
    self.certificate_bump = certificate_bump;
    self.student_name = student_name;
    self.student_grade = student_grade;
    self.course_name = course_name;
    self.school_name = school_name;
    self.school_uri = school_uri;
    self.issuer_name = issuer_name;
    self.issuer_role = issuer_role;
    self.issuer_uri = issuer_uri;
  }

}