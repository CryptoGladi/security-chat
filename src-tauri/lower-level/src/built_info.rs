pub(crate) mod raw {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(PartialEq)]
pub(crate) struct CrateInfo<'a> {
    pub(crate) name: &'a str,
    pub(crate) version: &'a str
}

impl<'a> CrateInfo<'a> {
    pub(crate) fn new(data: (&'a str, &'a str)) -> Self {
        Self {
            name: data.0,
            version: data.1
        }
    }
}

pub(crate) fn check_package(crate_for_check: CrateInfo) -> bool {
    for i in  raw::DEPENDENCIES.iter() {
        let crate_info = CrateInfo::new(*i);
        
        if crate_info == crate_for_check {
            return true;
        }
    }

    false
}
