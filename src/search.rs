use actix::prelude::*;
use elba::package::{manifest::PackageInfo, Name as PackageName};
use failure::Error;
use simsearch::SimSearch;

pub struct Search {
    engine: SimSearch<PackageName>,
}

impl Search {
    pub fn init() -> Result<Self, Error> {
        unimplemented!()
    }
}

impl Actor for Search {
    type Context = Context<Self>;
}

pub struct UpdateSearch {
    pub package_info: PackageInfo,
}

pub struct SearchPackage {
    pub query: String,
}

impl Message for UpdateSearch {
    type Result = Result<(), Error>;
}

impl Message for SearchPackage {
    type Result = Result<Vec<PackageName>, Error>;
}

impl Handler<UpdateSearch> for Search {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: UpdateSearch, _: &mut Self::Context) -> Self::Result {
        let terms: Vec<&str> = [msg.package_info.name.group(), msg.package_info.name.name()]
            .iter()
            .map(|s| *s)
            .chain(msg.package_info.keywords.iter().map(|s| s.as_str()))
            .collect();

        self.engine.insert(msg.package_info.name.clone(), &terms);

        Ok(())
    }
}

impl Handler<SearchPackage> for Search {
    type Result = Result<Vec<PackageName>, Error>;

    fn handle(&mut self, msg: SearchPackage, _: &mut Self::Context) -> Self::Result {
        Ok(self.engine.search(&msg.query))
    }
}
