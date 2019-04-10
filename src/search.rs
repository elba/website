use actix::prelude::*;
use elba::package::Name as PackageName;
use failure::Error;
use simsearch::SimSearch;

pub struct Search {
    engine: SimSearch<PackageName>,
}

impl Search {
    pub fn new() -> Self {
        Search {
            engine: SimSearch::new(),
        }
    }
}

impl Actor for Search {
    type Context = Context<Self>;
}

pub struct UpdateSearch {
    pub name: PackageName,
    pub keywords: Vec<String>,
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
        let terms: Vec<&str> = [msg.name.group(), msg.name.name()]
            .iter()
            .map(|s| *s)
            .chain(msg.keywords.iter().map(|s| s.as_str()))
            .collect();

        self.engine.insert(msg.name.clone(), &terms);

        Ok(())
    }
}

impl Handler<SearchPackage> for Search {
    type Result = Result<Vec<PackageName>, Error>;

    fn handle(&mut self, msg: SearchPackage, _: &mut Self::Context) -> Self::Result {
        Ok(self.engine.search(&msg.query))
    }
}
