# Standard Test Data Format (STDF) Specification

Version 4

---

## Table of Contents

- [Introduction to STDF](#introduction-to-stdf)
- [STDF Design Objectives](#stdf-design-objectives)
- [STDF Record Structure](#stdf-record-structure)
  - [STDF Record Header](#stdf-record-header)
  - [Record Types and Subtypes](#record-types-and-subtypes)
  - [Data Type Codes and Representation](#data-type-codes-and-representation)
  - [Note on Time and Date Usage](#note-on-time-and-date-usage)
  - [Optional Fields and Missing/Invalid Data](#optional-fields-and-missinginvalid-data)
- [STDF Record Types](#stdf-record-types)
  - [Note on "Initial Sequence"](#note-on-initial-sequence)
  - [Alphabetical Listing](#alphabetical-listing)
- [STDF Filenames](#stdf-filenames)
- [STDF File Ordering](#stdf-file-ordering)
- [Storing Repair Information](#storing-repair-information)
- [Using the Pin Mapping Records](#using-the-pin-mapping-records)
- [Differences Between STDF V3 and V4](#differences-between-stdf-v3-and-v4)
- [Glossary](#glossary)

---

## Introduction to STDF

As the ATE industry matures, many vendors offer networking systems that complement the test systems themselves and help customers get more out of their ATE investment. Many of these networking systems are converging on popular standards, such as Ethernet™.

A glaring hole in these standards has been the lack of test result data compatibility between test systems of different manufacturers, and sometimes within the product lines of a single manufacturer. In order to help overcome this problem, Teradyne has developed a simple, flexible, portable data format to which existing data files and formats can be easily and economically converted. Called the Standard Test Data Format (STDF™), its specification is contained in the following document.

---

## STDF Design Objectives

- Be capable of storing test data for all semiconductor testers and trimmers.
- Provide a common format for storage and transmission of data.
- Provide a basis for portable data reporting and analysis software.
- Decouple data message format and database format to allow enhancements to either, independently of the other.
- Provide support for optional (missing or invalid) data.
- Provide complete and concise documentation for developers and users.
- Make it easy for customers to write their own reports or reformat data for their own database.

---

## STDF Record Structure

### STDF Record Header

Each STDF record begins with a record header consisting of the following three fields:

| Field    | Description                                                                 |
|----------|-----------------------------------------------------------------------------|
| REC_LEN  | The number of bytes of data following the record header. REC_LEN does not include the four bytes of the record header. |
| REC_TYP  | An integer identifying a group of related STDF record types.                |
| REC_SUB  | An integer identifying a specific STDF record type within each REC_TYP group. |

### Record Types and Subtypes

The header of each STDF record contains a pair of fields called REC_TYP and REC_SUB. Each REC_TYP value identifies a group of related STDF record types. Each REC_SUB value identifies a single STDF record type within a REC_TYP group.

| REC_TYP Code | Meaning and STDF REC_SUB Codes                                                                 |
|--------------|-----------------------------------------------------------------------------------------------|
| 0            | Information about the STDF file<br>10 File Attributes Record (FAR)<br>20 Audit Trail Record (ATR) |
| 1            | Data collected on a per lot basis<br>10 Master Information Record (MIR)<br>20 Master Results Record (MRR)<br>30 Part Count Record (PCR)<br>40 Hardware Bin Record (HBR)<br>50 Software Bin Record (SBR)<br>60 Pin Map Record (PMR)<br>62 Pin Group Record (PGR)<br>63 Pin List Record (PLR)<br>70 Retest Data Record (RDR)<br>80 Site Description Record (SDR) |
| 2            | Data collected per wafer<br>10 Wafer Information Record (WIR)<br>20 Wafer Results Record (WRR)<br>30 Wafer Configuration Record (WCR) |
| 5            | Data collected on a per part basis<br>10 Part Information Record (PIR)<br>20 Part Results Record (PRR) |
| 10           | Data collected per test in the test program<br>30 Test Synopsis Record (TSR)                  |
| 15           | Data collected per test execution<br>10 Parametric Test Record (PTR)<br>15 Multiple-Result Parametric Record (MPR)<br>20 Functional Test Record (FTR) |
| 20           | Data collected per program segment<br>10 Begin Program Section Record (BPS)<br>20 End Program Section Record (EPS) |
| 50           | Generic Data<br>10 Generic Data Record (GDR)<br>30 Datalog Text Record (DTR)                    |
| 180          | Reserved for use by Image software                                                            |
| 181          | Reserved for use by IG900 software                                                             |

### Data Type Codes and Representation

| Code   | Description                                                                                     | C Type Specifier       |
|--------|-------------------------------------------------------------------------------------------------|------------------------|
| C*12   | Fixed length character string: If a fixed length character string does not fill the entire field, it must be left-justified and padded with spaces. | char [12]              |
| C*n    | Variable length character string: first byte = unsigned count of bytes to follow (maximum of 255 bytes) | char []                |
| C*f    | Variable length character string: string length is stored in another field                       | char []                |
| U*1    | One byte unsigned integer                                                                        | unsigned char          |
| U*2    | Two byte unsigned integer                                                                        | unsigned short         |
| U*4    | Four byte unsigned integer                                                                       | unsigned long          |
| I*1    | One byte signed integer                                                                          | char                   |
| I*2    | Two byte signed integer                                                                          | short                  |
| I*4    | Four byte signed integer                                                                         | long                   |
| R*4    | Four byte floating point number                                                                  | float                  |
| R*8    | Eight byte floating point number                                                                 | long float (double)    |
| B*6    | Fixed length bit-encoded data                                                                    | char [6]               |
| V*n    | Variable data type field: The data type is specified by a code in the first byte, and the data follows (maximum of 255 bytes) |                        |
| B*n    | Variable length bit-encoded field: First byte = unsigned count of bytes to follow (maximum of 255 bytes). First data item in least significant bit of the second byte of the array (first byte is count.) | char []                |
| D*n    | Variable length bit-encoded field: First two bytes = unsigned count of bits to follow (maximum of 65,535 bits). First data item in least significant bit of the third byte of the array (first two bytes are count). Unused bits at the high order end of the last byte must be zero. | char []                |
| N*1    | Unsigned integer data stored in a nibble. (Nibble = 4 bits of a byte). First item in low 4 bits, second item in high 4 bits. If an odd number of nibbles is indicated, the high nibble of the byte will be zero. Only whole bytes can be written to the STDF file. | char                   |
| kxTYPE | Array of data of the type specified. The value of 'k' (the number of elements in the array) is defined in an earlier field in the record. For example, an array of short unsigned integers is defined as kxU*2. | TYPE[]                 |

### Note on Time and Date Usage

The date and time field used in this specification is defined as a four byte (32 bit) unsigned integer field measuring the number of seconds since midnight on January 1st, 1970, in the local time zone. This is the UNIX standard base time, adjusted to the local time zone.

### Optional Fields and Missing/Invalid Data

| Data Type                     | Missing/Invalid Data Flag                                                                 |
|-------------------------------|------------------------------------------------------------------------------------------|
| Variable-length string        | Set the length byte to 0.                                                                 |
| Fixed-length character string | Fill the field with spaces.                                                               |
| Fixed-length binary string    | Set a flag bit in an Optional Data byte.                                                  |
| Time and date fields          | Use a binary 0.                                                                            |
| Signed and unsigned integers and floating point values | Use the indicated reserved value or set a flag bit in an Optional Data byte. |

---

## STDF Record Types

### Note on "Initial Sequence"

For several record types, the "Location" says that the record must appear "after the initial sequence." The phrase "initial sequence" refers to the records that must appear at the beginning of the STDF file. The requirements for the initial sequence are as follows:

- Every file must contain one File Attributes Record (FAR), one Master Information Record (MIR), one or more Part Count Records (PCR), and one Master Results Record (MRR). All other records are optional.
- The first record in the STDF file must be the File Attributes Record (FAR).
- If one or more Audit Trail Records (ATRs) are used, they must appear immediately after the FAR.
- The Master Information Record (MIR) must appear in every STDF file. Its location must be after the FAR and the ATRs (if ATRs are used).
- If the Retest Data Record (RDR) is used, it must appear immediately after the MIR.
- If one or more Site Description Records (SDRs) are used, they must appear immediately after the MIR and RDR (if the RDR is used).

### Alphabetical Listing

| Record Type | Page |
|-------------|------|
| ATR         | 17   |
| BPS         | 60   |
| DTR         | 64   |
| EPS         | 61   |
| FAR         | 16   |
| FTR         | 55   |
| GDR         | 62   |
| HBR         | 23   |
| MIR         | 18   |
| MPR         | 51   |
| MRR         | 21   |
| PCR         | 22   |
| PGR         | 29   |
| PIR         | 40   |
| PLR         | 30   |
| PMR         | 27   |
| PRR         | 41   |
| PTR         | 45   |
| RDR         | 32   |
| SBR         | 25   |
| SDR         | 33   |
| TSR         | 43   |
| WCR         | 38   |
| WIR         | 35   |
| WRR         | 36   |

---

## File Attributes Record (FAR)

**Function:** Contains the information necessary to determine how to decode the STDF data contained in the file.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (0)                             |                           |
| REC_SUB    | U*1       | Record sub-type (10)                        |                           |
| CPU_TYPE   | U*1       | CPU type that wrote this file               |                           |
| STDF_VER   | U*1       | STDF version number                         |                           |

**Notes on Specific Fields:**

- **CPU_TYPE:** Indicates which type of CPU wrote this STDF file. This information is useful for determining the CPU-dependent data representation of the integer and floating point fields in the file's records.


## Master Information Record (MIR)

**Function:** The MIR and the MRR (Master Results Record) contain all the global information that is to be stored for a tested lot of parts.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (1)                             |                           |
| REC_SUB    | U*1       | Record sub-type (10)                        |                           |
| SETUP_T    | U*4       | Date and time of job setup                  |                           |
| START_T    | U*4       | Date and time first part tested             |                           |
| STAT_NUM   | U*1       | Tester station number                       |                           |
| MODE_COD   | C*1       | Test mode code (e.g. prod, dev)             | space                     |
| RTST_COD   | C*1       | Lot retest code                             | space                     |
| PROT_COD   | C*1       | Data protection code                        | space                     |
| BURN_TIM   | U*2       | Burn-in time (in minutes)                   | 65,535                    |
| CMOD_COD   | C*1       | Command mode code                           | space                     |
| LOT_ID     | C*n       | Lot ID (customer specified)                 |                           |
| PART_TYP   | C*n       | Part Type (or product ID)                   |                           |
| NODE_NAM   | C*n       | Name of node that generated data            |                           |
| TSTR_TYP   | C*n       | Tester type                                 |                           |
| JOB_NAM    | C*n       | Job name (test program name)                |                           |
| JOB_REV    | C*n       | Job (test program) revision number          | length byte = 0           |
| SBLOT_ID   | C*n       | Sublot ID                                   | length byte = 0           |
| OPER_NAM   | C*n       | Operator name or ID (at setup time)         | length byte = 0           |
| EXEC_TYP   | C*n       | Tester executive software type              | length byte = 0           |
| EXEC_VER   | C*n       | Tester exec software version number         | length byte = 0           |
| TEST_COD   | C*n       | Test phase or step code                     | length byte = 0           |
| TST_TEMP   | C*n       | Test temperature                            | length byte = 0           |
| USER_TXT   | C*n       | Generic user text                           | length byte = 0           |
| AUX_FILE   | C*n       | Name of auxiliary data file                 | length byte = 0           |
| PKG_TYP    | C*n       | Package type                                | length byte = 0           |
| FAMLY_ID   | C*n       | Product family ID                           | length byte = 0           |
| DATE_COD   | C*n       | Date code                                   | length byte = 0           |
| FACIL_ID   | C*n       | Test facility ID                            | length byte = 0           |
| FLOOR_ID   | C*n       | Test floor ID                               | length byte = 0           |
| PROC_ID    | C*n       | Fabrication process ID                      | length byte = 0           |
| OPER_FRQ   | C*n       | Operation frequency or step                 | length byte = 0           |
| SPEC_NAM   | C*n       | Test specification name                     | length byte = 0           |
| SPEC_VER   | C*n       | Test specification version number           | length byte = 0           |
| FLOW_ID    | C*n       | Test flow ID                                | length byte = 0           |
| SETUP_ID   | C*n       | Test setup ID                               | length byte = 0           |
| DSGN_REV   | C*n       | Device design revision                      | length byte = 0           |
| ENG_ID     | C*n       | Engineering lot ID                          | length byte = 0           |
| ROM_COD    | C*n       | ROM code ID                                 | length byte = 0           |
| SERL_NUM   | C*n       | Tester serial number                        | length byte = 0           |
| SUPR_NAM   | C*n       | Supervisor name or ID                       | length byte = 0           |

---

## Master Results Record (MRR)

**Function:** The Master Results Record (MRR) is a logical extension of the Master Information Record (MIR).

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (1)                             |                           |
| REC_SUB    | U*1       | Record sub-type (20)                        |                           |
| FINISH_T   | U*4       | Date and time last part tested              |                           |
| DISP_COD   | C*1       | Lot disposition code                        | space                     |
| USR_DESC   | C*n       | Lot description supplied by user            | length byte = 0           |
| EXC_DESC   | C*n       | Lot description supplied by exec            | length byte = 0           |

---

## Part Count Record (PCR)

**Function:** Contains the part count totals for one or all test sites.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (1)                             |                           |
| REC_SUB    | U*1       | Record sub-type (30)                        |                           |
| HEAD_NUM   | U*1       | Test head number                            | See note                  |
| SITE_NUM   | U*1       | Test site number                            |                           |
| PART_CNT   | U*4       | Number of parts tested                      |                           |
| RTST_CNT   | U*4       | Number of parts retested                    | 4,294,967,295              |
| ABRT_CNT   | U*4       | Number of aborts during testing             | 4,294,967,295              |
| GOOD_CNT   | U*4       | Number of good (passed) parts tested        | 4,294,967,295              |
| FUNC_CNT   | U*4       | Number of functional parts tested           | 4,294,967,295              |

---

## Hardware Bin Record (HBR)

**Function:** Stores a count of the parts "physically" placed in a particular bin after testing.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (1)                             |                           |
| REC_SUB    | U*1       | Record sub-type (40)                        |                           |
| HEAD_NUM   | U*1       | Test head number                            | See note                  |
| SITE_NUM   | U*1       | Test site number                            |                           |
| HBIN_NUM   | U*2       | Hardware bin number                         |                           |
| HBIN_CNT   | U*4       | Number of parts in bin                      |                           |
| HBIN_PF    | C*1       | Pass/fail indication                        | space                     |
| HBIN_NAM   | C*n       | Name of hardware bin                        | length byte = 0           |

---

## Software Bin Record (SBR)

**Function:** Stores a count of the parts associated with a particular logical bin after testing.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (1)                             |                           |
| REC_SUB    | U*1       | Record sub-type (50)                        |                           |
| HEAD_NUM   | U*1       | Test head number                            | See note                  |
| SITE_NUM   | U*1       | Test site number                            |                           |
| SBIN_NUM   | U*2       | Software bin number                         |                           |
| SBIN_CNT   | U*4       | Number of parts in bin                      |                           |
| SBIN_PF    | C*1       | Pass/fail indication                        | space                     |
| SBIN_NAM   | C*n       | Name of software bin                        | length byte = 0           |

---

## Pin Map Record (PMR)

**Function:** Provides indexing of tester channel names, and maps them to physical and logical pin names.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (1)                             |                           |
| REC_SUB    | U*1       | Record sub-type (60)                        |                           |
| PMR_INDX   | U*2       | Unique index associated with pin            |                           |
| CHAN_TYP   | U*2       | Channel type                                | 0                         |
| CHAN_NAM   | C*n       | Channel name                                | length byte = 0           |
| PHY_NAM    | C*n       | Physical name of pin                        | length byte = 0           |
| LOG_NAM    | C*n       | Logical name of pin                         | length byte = 0           |
| HEAD_NUM   | U*1       | Head number associated with channel         | 1                         |
| SITE_NUM   | U*1       | Site number associated with channel         | 1                         |

---

## Pin Group Record (PGR)

**Function:** Associates a name with a group of pins.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (1)                             |                           |
| REC_SUB    | U*1       | Record sub-type (62)                        |                           |
| GRP_INDX   | U*2       | Unique index associated with pin group      |                           |
| GRP_NAM    | C*n       | Name of pin group                           | length byte = 0           |
| INDX_CNT   | U*2       | Count (k) of PMR indexes                     |                           |
| PMR_INDX   | kxU*2     | Array of indexes for pins in the group      | INDX_CNT = 0              |

---

## Pin List Record (PLR)

**Function:** Defines the current display radix and operating mode for a pin or pin group.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (1)                             |                           |
| REC_SUB    | U*1       | Record sub-type (63)                        |                           |
| GRP_CNT    | U*2       | Count (k) of pins or pin groups              |                           |
| GRP_INDX   | kxU*2     | Array of pin or pin group indexes            |                           |
| GRP_MODE   | kxU*2     | Operating mode of pin group                 | 0                         |
| GRP_RADX   | kxU*1     | Display radix of pin group                  | 0                         |
| PGM_CHAR   | kxC*n     | Program state encoding characters           | length byte = 0           |
| RTN_CHAR   | kxC*n     | Return state encoding characters            | length byte = 0           |

---

## Retest Data Record (RDR)

**Function:** Signals that the data in this STDF file is for retested parts.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (1)                             |                           |
| REC_SUB    | U*1       | Record sub-type (70)                        |                           |
| NUM_BINS   | U*2       | Number (k) of bins being retested           |                           |
| RTST_BIN   | kxU*2     | Array of retest bin numbers                 | NUM_BINS = 0              |

---

## Site Description Record (SDR)

**Function:** Contains the configuration information for one or more test sites, connected to one test head, that compose a site group.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (1)                             |                           |
| REC_SUB    | U*1       | Record sub-type (80)                        |                           |
| HEAD_NUM   | U*1       | Test head number                            |                           |
| SITE_GRP   | U*1       | Site group number                           |                           |
| SITE_CNT   | U*1       | Number (k) of test sites in site group      |                           |
| SITE_NUM   | kxU*1     | Array of test site numbers                  |                           |
| HAND_TYP   | C*n       | Handler or prober type                      | length byte = 0           |
| HAND_ID    | C*n       | Handler or prober ID                        | length byte = 0           |
| CARD_TYP   | C*n       | Probe card type                             | length byte = 0           |
| CARD_ID    | C*n       | Probe card ID                               | length byte = 0           |
| LOAD_TYP   | C*n       | Load board type                             | length byte = 0           |
| LOAD_ID    | C*n       | Load board ID                               | length byte = 0           |
| DIB_TYP    | C*n       | DIB board type                              | length byte = 0           |
| DIB_ID     | C*n       | DIB board ID                                | length byte = 0           |
| CABL_TYP   | C*n       | Interface cable type                        | length byte = 0           |
| CABL_ID    | C*n       | Interface cable ID                          | length byte = 0           |
| CONT_TYP   | C*n       | Handler contactor type                      | length byte = 0           |
| CONT_ID    | C*n       | Handler contactor ID                        | length byte = 0           |
| LASR_TYP   | C*n       | Laser type                                  | length byte = 0           |
| LASR_ID    | C*n       | Laser ID                                    | length byte = 0           |
| EXTR_TYP   | C*n       | Extra equipment type field                  | length byte = 0           |
| EXTR_ID    | C*n       | Extra equipment ID                          | length byte = 0           |

---

## Wafer Information Record (WIR)

**Function:** Acts mainly as a marker to indicate where testing of a particular wafer begins for each wafer tested by the job plan.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (2)                             |                           |
| REC_SUB    | U*1       | Record sub-type (10)                        |                           |
| HEAD_NUM   | U*1       | Test head number                            |                           |
| SITE_GRP   | U*1       | Site group number                           | 255                       |
| START_T    | U*4       | Date and time first part tested             |                           |
| WAFER_ID   | C*n       | Wafer ID                                    | length byte = 0           |

---

## Wafer Results Record (WRR)

**Function:** Contains the result information relating to each wafer tested by the job plan.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (2)                             |                           |
| REC_SUB    | U*1       | Record sub-type (20)                        |                           |
| HEAD_NUM   | U*1       | Test head number                            |                           |
| SITE_GRP   | U*1       | Site group number                           | 255                       |
| FINISH_T   | U*4       | Date and time last part tested              |                           |
| PART_CNT   | U*4       | Number of parts tested                      |                           |
| RTST_CNT   | U*4       | Number of parts retested                    | 4,294,967,295              |
| ABRT_CNT   | U*4       | Number of aborts during testing             | 4,294,967,295              |
| GOOD_CNT   | U*4       | Number of good (passed) parts tested        | 4,294,967,295              |
| FUNC_CNT   | U*4       | Number of functional parts tested           | 4,294,967,295              |
| WAFER_ID   | C*n       | Wafer ID                                    | length byte = 0           |
| FABWF_ID   | C*n       | Fab wafer ID                                | length byte = 0           |
| FRAME_ID   | C*n       | Wafer frame ID                              | length byte = 0           |
| MASK_ID    | C*n       | Wafer mask ID                               | length byte = 0           |
| USR_DESC   | C*n       | Wafer description supplied by user          | length byte = 0           |
| EXC_DESC   | C*n       | Wafer description supplied by exec          | length byte = 0           |

---

## Wafer Configuration Record (WCR)

**Function:** Contains the configuration information for the wafers tested by the job plan.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (2)                             |                           |
| REC_SUB    | U*1       | Record sub-type (30)                        |                           |
| WAFR_SIZ   | R*4       | Diameter of wafer in WF_UNITS                | 0                         |
| DIE_HT     | R*4       | Height of die in WF_UNITS                    | 0                         |
| DIE_WID    | R*4       | Width of die in WF_UNITS                     | 0                         |
| WF_UNITS   | U*1       | Units for wafer and die dimensions           | 0                         |
| WF_FLAT    | C*1       | Orientation of wafer flat                   | space                     |
| CENTER_X   | I*2       | X coordinate of center die on wafer         | -32768                    |
| CENTER_Y   | I*2       | Y coordinate of center die on wafer         | -32768                    |
| POS_X      | C*1       | Positive X direction of wafer               | space                     |
| POS_Y      | C*1       | Positive Y direction of wafer               | space                     |

---

## Part Information Record (PIR)

**Function:** Acts as a marker to indicate where testing of a particular part begins for each part tested by the test program.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (5)                             |                           |
| REC_SUB    | U*1       | Record sub-type (10)                        |                           |
| HEAD_NUM   | U*1       | Test head number                            |                           |
| SITE_NUM   | U*1       | Test site number                            |                           |

---

## Part Results Record (PRR)

**Function:** Contains the result information relating to each part tested by the test program.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (5)                             |                           |
| REC_SUB    | U*1       | Record sub-type (20)                        |                           |
| HEAD_NUM   | U*1       | Test head number                            |                           |
| SITE_NUM   | U*1       | Test site number                            |                           |
| PART_FLG   | B*1       | Part information flag                       |                           |
| NUM_TEST   | U*2       | Number of tests executed                    |                           |
| HARD_BIN   | U*2       | Hardware bin number                         |                           |
| SOFT_BIN   | U*2       | Software bin number                         | 65535                     |
| X_COORD    | I*2       | (Wafer) X coordinate                         | -32768                    |
| Y_COORD    | I*2       | (Wafer) Y coordinate                         | -32768                    |
| TEST_T     | U*4       | Elapsed test time in milliseconds            | 0                         |
| PART_ID    | C*n       | Part identification                         | length byte = 0           |
| PART_TXT   | C*n       | Part description text                       | length byte = 0           |
| PART_FIX   | B*n       | Part repair information                     | length byte = 0           |

---

## Test Synopsis Record (TSR)

**Function:** Contains the test execution and failure counts for one parametric or functional test in the test program.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (10)                            |                           |
| REC_SUB    | U*1       | Record sub-type (30)                        |                           |
| HEAD_NUM   | U*1       | Test head number                            | See note                  |
| SITE_NUM   | U*1       | Test site number                            |                           |
| TEST_TYP   | C*1       | Test type                                   | space                     |
| TEST_NUM   | U*4       | Test number                                 |                           |
| EXEC_CNT   | U*4       | Number of test executions                   | 4,294,967,295              |
| FAIL_CNT   | U*4       | Number of test failures                     | 4,294,967,295              |
| ALRM_CNT   | U*4       | Number of alarmed tests                     | 4,294,967,295              |
| TEST_NAM   | C*n       | Test name                                   | length byte = 0           |
| SEQ_NAME   | C*n       | Sequencer (program segment/flow) name       | length byte = 0           |
| TEST_LBL   | C*n       | Test label or text                          | length byte = 0           |
| OPT_FLAG   | B*1       | Optional data flag                          | See note                  |
| TEST_TIM   | R*4       | Average test execution time in seconds      | OPT_FLAG bit 2 = 1        |
| TEST_MIN   | R*4       | Lowest test result value                    | OPT_FLAG bit 0 = 1        |
| TEST_MAX   | R*4       | Highest test result value                   | OPT_FLAG bit 1 = 1        |
| TST_SUMS   | R*4       | Sum of test result values                   | OPT_FLAG bit 4 = 1        |
| TST_SQRS   | R*4       | Sum of squares of test result values        | OPT_FLAG bit 5 = 1        |

---

## Parametric Test Record (PTR)

**Function:** Contains the results of a single execution of a parametric test in the test program.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (15)                            |                           |
| REC_SUB    | U*1       | Record sub-type (10)                        |                           |
| TEST_NUM   | U*4       | Test number                                 |                           |
| HEAD_NUM   | U*1       | Test head number                            |                           |
| SITE_NUM   | U*1       | Test site number                            |                           |
| TEST_FLG   | B*1       | Test flags (fail, alarm, etc.)              |                           |
| PARM_FLG   | B*1       | Parametric test flags (drift, etc.)         |                           |
| RESULT     | R*4       | Test result                                 | TEST_FLG bit 1 = 1        |
| TEST_TXT   | C*n       | Test description text or label              | length byte = 0           |
| ALARM_ID   | C*n       | Name of alarm                               | length byte = 0           |
| OPT_FLAG   | B*1       | Optional data flag                          | See note                  |
| RES_SCAL   | I*1       | Test results scaling exponent               | OPT_FLAG bit 0 = 1        |
| LLM_SCAL   | I*1       | Low limit scaling exponent                  | OPT_FLAG bit 4 or 6 = 1   |
| HLM_SCAL   | I*1       | High limit scaling exponent                 | OPT_FLAG bit 5 or 7 = 1   |
| LO_LIMIT   | R*4       | Low test limit value                        | OPT_FLAG bit 4 or 6 = 1   |
| HI_LIMIT   | R*4       | High test limit value                       | OPT_FLAG bit 5 or 7 = 1   |
| UNITS      | C*n       | Test units                                  | length byte = 0           |
| C_RESFMT   | C*n       | ANSI C result format string                 | length byte = 0           |
| C_LLMFMT   | C*n       | ANSI C low limit format string              | length byte = 0           |
| C_HLMFMT   | C*n       | ANSI C high limit format string             | length byte = 0           |
| LO_SPEC    | R*4       | Low specification limit value               | OPT_FLAG bit 2 = 1        |
| HI_SPEC    | R*4       | High specification limit value              | OPT_FLAG bit 3 = 1        |

---

## Multiple-Result Parametric Record (MPR)

**Function:** Contains the results of a single execution of a parametric test in the test program where that test returns multiple values.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (15)                            |                           |
| REC_SUB    | U*1       | Record sub-type (15)                        |                           |
| TEST_NUM   | U*4       | Test number                                 |                           |
| HEAD_NUM   | U*1       | Test head number                            |                           |
| SITE_NUM   | U*1       | Test site number                            |                           |
| TEST_FLG   | B*1       | Test flags (fail, alarm, etc.)              |                           |
| PARM_FLG   | B*1       | Parametric test flags (drift, etc.)         |                           |
| RTN_ICNT   | U*2       | Count (j) of PMR indexes                     | See note                  |
| RSLT_CNT   | U*2       | Count (k) of returned results                | See note                  |
| RTN_STAT   | jxN*1     | Array of returned states                    | RTN_ICNT = 0              |
| RTN_RSLT   | kxR*4     | Array of returned results                   | RSLT_CNT = 0              |
| TEST_TXT   | C*n       | Descriptive text or label                   | length byte = 0           |
| ALARM_ID   | C*n       | Name of alarm                               | length byte = 0           |
| OPT_FLAG   | B*1       | Optional data flag                          | See note                  |
| RES_SCAL   | I*1       | Test result scaling exponent                | OPT_FLAG bit 0 = 1        |
| LLM_SCAL   | I*1       | Test low limit scaling exponent             | OPT_FLAG bit 4 or 6 = 1   |
| HLM_SCAL   | I*1       | Test high limit scaling exponent            | OPT_FLAG bit 5 or 7 = 1   |
| LO_LIMIT   | R*4       | Test low limit value                        | OPT_FLAG bit 4 or 6 = 1   |
| HI_LIMIT   | R*4       | Test high limit value                       | OPT_FLAG bit 5 or 7 = 1   |
| START_IN   | R*4       | Starting input value (condition)            | OPT_FLAG bit 1 = 1        |
| INCR_IN    | R*4       | Increment of input condition                | OPT_FLAG bit 1 = 1        |
| RTN_INDX   | jxU*2     | Array of PMR indexes                        | RTN_ICNT = 0              |
| UNITS      | C*n       | Units of returned results                   | length byte = 0           |
| UNITS_IN   | C*n       | Input condition units                       | length byte = 0           |
| C_RESFMT   | C*n       | ANSI C result format string                 | length byte = 0           |
| C_LLMFMT   | C*n       | ANSI C low limit format string              | length byte = 0           |
| C_HLMFMT   | C*n       | ANSI C high limit format string             | length byte = 0           |
| LO_SPEC    | R*4       | Low specification limit value               | OPT_FLAG bit 2 = 1        |
| HI_SPEC    | R*4       | High specification limit value              | OPT_FLAG bit 3 = 1        |

---

## Functional Test Record (FTR)

**Function:** Contains the results of the single execution of a functional test in the test program.

| Field Name | Data Type | Field Description                          | Missing/Invalid Data Flag |
|------------|-----------|---------------------------------------------|---------------------------|
| REC_LEN    | U*2       | Bytes of data following header              |                           |
| REC_TYP    | U*1       | Record type (15)                            |                           |
| REC_SUB    | U*1       | Record sub-type (20)                        |                           |
| TEST_NUM   | U*4       | Test number                                 |                           |
| HEAD_NUM   | U*1       | Test head number                            |                           |
| SITE_NUM   | U*1       | Test site number                            |                           |
| TEST_FLG   | B*1       | Test flags (fail, alarm, etc.)              |                           |
| OPT_FLAG   | B*1       | Optional data flag                          | See note                  |
| CYCL_CNT   | U*4       | Cycle count of vector                       | OPT_FLAG bit 0 = 1        |
| REL_VADR   | U*4       | Relative vector address                     | OPT_FLAG bit 1 = 1        |
| REPT_CNT   | U*4       | Repeat count of vector                      | OPT_FLAG bit 2 = 1        |
| NUM_FAIL   | U*4       | Number of pins with 1 or more failures      | OPT_FLAG bit 3 = 1        |
| XFAIL_AD   | I*4       | X logical device failure address            | OPT_FLAG bit 4 = 1        |
| YFAIL_AD   | I*4       | Y logical device failure address            | OPT_FLAG bit 4 = 1        |
| VECT_OFF   | I*2       | Offset from vector of interest              | OPT_FLAG bit 5 = 1        |
| RTN_ICNT   | U*2       | Count (j) of return data PMR indexes         | See note                  |
| PGM_ICNT   | U*2       | Count (k) of programmed state indexes        | See note                  |
| RTN_INDX   | jxU*2     | Array of return data PMR indexes            | RTN_ICNT = 0              |
| RTN_STAT   | jxN*1     | Array of returned states                    | RTN_ICNT = 0              |
| PGM_INDX   | kxU*2     | Array of programmed state indexes           | PGM_ICNT = 0              |
| PGM_STAT   | kxN*1     | Array of programmed states                  | PGM_ICNT = 0              |
| FAIL_PIN   | D*n       | Failing pin bitfield                        | length bytes = 0          |
| VECT_NAM   | C*n       | Vector module pattern name                  | length byte = 0           |
| TIME_SET   | C*n       | Time set name                               | length byte = 0           |
| OP_CODE    | C*n       | Vector Op Code                              | length byte = 0           |
| TEST_TXT   | C*n       | Descriptive text or label                   | length byte = 0           |
| ALARM_ID   | C*n       | Name of alarm                               | length byte = 0           |
| PROG_TXT   | C*n       | Additional programmed information           | length byte = 0           |
| RSLT_TXT   | C*n       | Additional result information               | length byte = 0           |
| PATG_NUM   | U*1       | Pattern generator number                    | 255                       |
| SPIN_MAP   | D*n       | Bit map of enabled comparators              | length byte = 0           |

**Notes on Specific Fields:**

- **GEN_DATA:** Is repeated FLD_CNT number of times. Each GEN_DATA field consists of a data type code followed by the actual data.
