use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Merge 1 church_windows_csv and 1 onrealm_csv into a single working-list formated csv
#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(short, long, parse(from_os_str))]
    church_windows_csv: PathBuf,

    #[structopt(short = "r", long, parse(from_os_str))]
    onrealm_csv: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    output_csv: PathBuf,

    #[structopt(short, long, parse(try_from_str = regex::Regex::new), default_value = "/^")]
    filter_names: regex::Regex,
}

// 	Alumni Last Name	Alumni First Name	Member Status	Family Position	Has a Pledge	Spouse First Name	Email	Phone Number	Cell Phone	Street Address	Street  2	City	State	Zip	Source	Last updated	Notes	Referred By	Membership date	Time Away	Pastoral	Virtual	In Person	RE Family	Constant Contact Y/N	Social Justice	Operating Budget	Capital Campaign					
#[derive(Serialize, Deserialize, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct WorkingListEntry {
    // need a field for the names in the first column. lets just call it "admin" and call it a day
    #[serde(rename = "admin")]
    admin: String,

    #[serde(rename = "Alumni Last Name")]
    name_last: String,
    #[serde(rename = "Alumni First Name")]
    name_first: String,

    #[serde(rename = "Member Status")]
    member_status: String,

    #[serde(rename = "Family Position")]
    family_position: String,

    #[serde(rename = "Has a Pledge")]
    has_pledge: String,

    #[serde(rename = "Spouse First Name")]
    spouse_first_name: String,
    #[serde(rename = "Email")]
    email: String,
    #[serde(rename = "Phone Number")]
    phone_number: String,
    #[serde(rename = "Cell Phone")]
    cell_phone: String,

    #[serde(rename = "Street Address")]
    street: String,

    #[serde(rename = "Street 2")]
    street2: String,

    #[serde(rename = "City")]
    city: String,
    #[serde(rename = "State")]
    state: String,
    #[serde(rename = "Zip")]
    zip: String,

    #[serde(rename = "Source")]
    source: String,

    #[serde(rename = "Last updated")]
    last_updated: String,

    #[serde(rename = "Notes")]
    notes: String,

    #[serde(rename = "Referred By")]
    referred_by: String,

    #[serde(rename = "Membership date")]
    membership_date: String,
    #[serde(rename = "Time Away")]
    time_away: String,
    #[serde(rename = "Pastoral")]
    pastoral: String,
    #[serde(rename = "Virtual")]
    virtual_: String,
    #[serde(rename = "In Person")]
    in_person: String,
    #[serde(rename = "RE Family")]
    re_family: String,
    #[serde(rename = "Constant Contact Y/N")]
    constant_contact: String,
    #[serde(rename = "Social Justice")]
    social_justice: String,
    #[serde(rename = "Operating Budget")]
    operating_budget: String,
    #[serde(rename = "Capital Campaign")]
    capital_campaign: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChurchWindowsRecord {
    #[serde(rename = "Title")]
    title: String,

    #[serde(rename = "firstName")]
    name_first: String,

    #[serde(rename = "LastName")]
    name_last: String,

    #[serde(rename = "MailingLabel")]
    mailing_label: String,

    #[serde(rename = "AddressBlock")]
    address_block: String,

    #[serde(rename = "Address1")]
    addr1: String,

    #[serde(rename = "Address2")]
    addr2: String,

    #[serde(rename = "CityState")]
    city_state: String,

    #[serde(rename = "Zip")]
    zip: String,

    #[serde(rename = "Membershipdate")]
    membership_date: String,

    #[serde(rename = "eMail")]
    email: String,

    #[serde(rename = "HomePhone")]
    home_phone: String,

    #[serde(rename = "CellPhone")]
    cell_phone: String,

    #[serde(rename = "LastUpdate")]
    last_update: String,

    #[serde(rename = "Status")]
    status: String,

    #[serde(rename = "BirthDate")]
    birth_date: String,
}

fn read_church_windows(church_windows_path: &Path) -> Result<Vec<ChurchWindowsRecord>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(church_windows_path)?;
    let mut first = true;
    let mut headers = None;
    let mut res = Vec::with_capacity(1024);
    for result in rdr.records() {
        let record = result?;
        if first {
            // church windows has 2 headers, default is 1 header. Actual header is second one
            first = false;
            headers = Some(record);
            continue;
        }

        let de_rec = record.deserialize(headers.as_ref())?;

        res.push(de_rec);
    }

    Ok(res)
}


#[derive(Serialize, Deserialize, Debug)]
struct OnrealmRecord {
    // we don't care about these fields, but we need placeholders to skip over them as we
    // deserialize by index
    vlookup_name: String,
    vlookup_email: String,
    vlookup_phone: String,
    vlookup_mobile: String,

    #[serde(rename = "Individual Id")]
    individual_id: String,

    #[serde(rename = "Label")]
    label: String,
    
    #[serde(rename = "First Name")]
    name_first: String,

    #[serde(rename = "Current Pledge")]
    current_pledge: String,

    #[serde(rename = "Last Name")]
    name_last: String,

    #[serde(rename = "Primary Email")]
    primary_email: String,

    #[serde(rename = "Family Id")]
    family_id: String,

    #[serde(rename = "Primary Phone Number")]
    primary_phone_number: String,

    #[serde(rename = "Title")]
    title: String,

    #[serde(rename = "First Name")]
    name_first_2: String,

    #[serde(rename = "Last Name")]
    name_last_2: String,

    #[serde(rename = "Individual Status")]
    individual_status: String,

    #[serde(rename = "Address Line 1 (Primary)")]
    addr_line_1: String,

    #[serde(rename = "Address Line 2 (Primary)")]
    addr_line_2: String,

    #[serde(rename = "Address City (Primary)")]
    addr_city: String,

    #[serde(rename = "Address Postal Code (Primary)")]
    postal_code: String,

    #[serde(rename = "Address State (Primary)")]
    addr_state: String,

    #[serde(rename = "Membership Date")]
    membership_date: String,

    #[serde(rename = "Primary Email Address")]
    email_2: String,

    #[serde(rename = "Alternate Email Address")]
    email_3: String,

    #[serde(rename = "Home Phone Number")]
    home_phone: String,

    #[serde(rename = "Mobile Phone Number")]
    mobile_phone: String,

    #[serde(rename = "Date of Birth")]
    dob: String,

    #[serde(rename = "Marital Status")]
    marital_status: String,

    #[serde(rename = "Family Position")]
    family_position: String,

    #[serde(rename = "Pronouns")]
    pronouns: String,

    #[serde(rename = "Member Status")]
    member_status: String,
}

// StringRecord(["vLookup Name", "vLookup eMail", "vLookup Phone", "vLookup Mobile", "Individual Id", "Label", "First Name", "Current Pledge", "Last Name", "Primary Email", "Family Id", "Primary Phone Number", "Title", "First Name", "Last Name", "Individual Status", "Address Line 1 (Primary)", "Address Line 2 (Primary)", "Address City (Primary)", "Address Postal Code (Primary)", "Address State (Primary)", "Membership Date", "Primary Email Address", "Alternate Email Address", "Home Phone Number", "Mobile Phone Number", "Date of Birth", "Marital Status", "Family Position", "Pronouns", "Member Status"])

fn read_onrealm(onrealm_csv_path: &Path) -> Result<Vec<OnrealmRecord>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(onrealm_csv_path)?;
    let mut first = true;
    let mut res = Vec::with_capacity(1024);
    for result in rdr.records() {
        let record = result?;
        if first {
            first = false;
            continue;
        }

        // we can't use headers because there are duplicate columns. Use position instead
        let de_rec = record.deserialize(None)?;

        res.push(de_rec);
    }

    Ok(res)
}

impl From<OnrealmRecord> for WorkingListEntry {
    fn from(orr: OnrealmRecord) -> Self {
        Self {
            admin: "auto".to_owned(),

            // NOTE: orr includes duplicate name_last_2 and name_first_2, which we're ignoring
            name_first: orr.name_first,
            name_last: orr.name_last,

            family_position: orr.family_position,
            spouse_first_name: "".to_owned(),

            member_status: orr.member_status,
            membership_date: "".to_owned(),
            has_pledge: "".to_owned(),
            referred_by: "".to_owned(),
            time_away: "".to_owned(),

            // NOTE: orr includes a `home_phone` which we're discarding
            cell_phone: orr.mobile_phone,
            phone_number: orr.primary_phone_number,

            street: orr.addr_line_1,
            street2: orr.addr_line_2,
            city: orr.addr_city,
            state: orr.addr_state,
            zip: orr.postal_code,

            source: "REALM".to_owned(),

            // NOTE: orr incldues 3 emails. We're picking the first.
            email: orr.primary_email,

            notes: "".to_owned(),

            re_family: "".to_owned(),
            last_updated: "".to_owned(),
            virtual_: "".to_owned(),
            in_person: "".to_owned(),
            pastoral: "".to_owned(),
            constant_contact: "".to_owned(),
            social_justice: "".to_owned(),
            operating_budget: "".to_owned(),
            capital_campaign: "".to_owned(),
        }
    }
}

impl From<ChurchWindowsRecord> for WorkingListEntry {
    fn from(cwr: ChurchWindowsRecord) -> Self {
        let (city, state) = {
            let cs = cwr.city_state.clone();
            if cs.is_empty() {
                ("".to_owned(), "".to_owned())
            } else {
                // FIXME: some entries include a comma between the city and the state
                let cs: Vec<&str> = cs.rsplitn(2, " ").collect();
                if cs.len() == 1 {
                    // probably just a town (state omitted)
                    // state is _probably_ NJ, but lets just leave it unset
                    //
                    // FIXME: some entries omit the state instead of the city.
                    (cs[0].to_owned(), "".to_owned())
                } else if cs.len() != 2 {
                    panic!("cs: {:?}, cwr: {:?}", cs, cwr);
                } else {
                    // FIXME: we're assuming we've got a state and a city, but we might have a city
                    // which has a space in it. Solution is probably trying to recognize state
                    // names.
                    //
                    // FIXME: some entries include a zip code following the state
                    (cs[1].to_owned(), cs[0].trim().to_owned())
                }
            }
        };
        Self {
            admin: "auto".to_owned(),

            name_first: cwr.name_first,
            name_last: cwr.name_last,

            family_position: "".to_owned(),
            spouse_first_name: "".to_owned(),

            member_status: cwr.status,
            membership_date: cwr.membership_date,
            has_pledge: "".to_owned(),
            referred_by: "".to_owned(),
            time_away: "".to_owned(),

            // NOTE: orr includes a `home_phone` which we're discarding
            cell_phone: cwr.cell_phone,
            phone_number: cwr.home_phone,

            street: cwr.addr1,
            street2: cwr.addr2,
            city,
            state,
            zip: cwr.zip,

            source: "CW".to_owned(),

            // NOTE: orr incldues 3 emails. We're picking the first.
            email: cwr.email,

            notes: "".to_owned(),

            re_family: "".to_owned(),
            last_updated: cwr.last_update,
            virtual_: "".to_owned(),
            in_person: "".to_owned(),
            pastoral: "".to_owned(),
            constant_contact: "".to_owned(),
            social_justice: "".to_owned(),
            operating_budget: "".to_owned(),
            capital_campaign: "".to_owned(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();
    println!("{:?}", opts);

    let cw = read_church_windows(&opts.church_windows_csv)?;

    /*
    for cw_e in cw {
        println!("{:?}", cw_e);
    }
    */

    let onrealm = read_onrealm(&opts.onrealm_csv)?;

    /*
    for or_e in onrealm {
        println!("{:?}", or_e);
    }
    */

    // merge the records!
    
    // for now, just do the really simple "create records for all of them"
    let mut all: BTreeMap<(String, String), WorkingListEntry> = BTreeMap::new();

    for cw_e in cw {
        let v: WorkingListEntry = cw_e.into();
        let k = (v.name_last.clone(), v.name_first.clone());
        all.insert(k, v);
    }

    for or_e in onrealm {
        let v: WorkingListEntry = or_e.into();
        let k = (v.name_last.clone(), v.name_first.clone());
        all.insert(k, v);
    }

    let mut out = csv::Writer::from_path(opts.output_csv)?;

    for ((name_last, _), v) in all {
        if opts.filter_names.is_match(&name_last) {
            continue;
        }
        out.serialize(v)?;
    }

    out.flush()?;

    Ok(())
}
