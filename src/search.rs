use actix::prelude::*;
use elba::package::{manifest::PackageInfo, Name as PackageName};
use failure::Error;
use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::query::{BooleanQuery, FuzzyTermQuery, Occur, Query, TermQuery};
use tantivy::schema::*;
use tantivy::{DocAddress, Score};
use tantivy::{Index, IndexReader, IndexWriter};

use crate::CONFIG;

pub struct SearchEngine {
    index: Index,
    fields: Fields,
    reader: IndexReader,
}

pub struct Fields {
    pub group_package: Field,
    pub group: Field,
    pub pacakge: Field,
    pub description: Field,
    pub keywords: Field,
}

impl SearchEngine {
    pub fn init() -> Result<Self, Error> {
        let mut schema_builder = SchemaBuilder::default();

        let group_package = schema_builder.add_text_field("group_package", TEXT | STORED);
        let group = schema_builder.add_text_field("group", TEXT | STORED);
        let pacakge = schema_builder.add_text_field("pacakge", TEXT | STORED);
        let description = schema_builder.add_text_field("description", TEXT | STORED);
        let keywords = schema_builder.add_text_field("keywords", TEXT | STORED);

        let fields = Fields {
            group_package,
            group,
            pacakge,
            description,
            keywords,
        };

        let schema = schema_builder.build();

        let index = Index::open_or_create(
            MmapDirectory::open(&CONFIG.search_engine_path)?,
            schema.clone(),
        )?;

        let reader = index.reader()?;

        Ok(SearchEngine {
            index,
            fields,
            reader,
        })
    }
}

impl Actor for SearchEngine {
    type Context = Context<Self>;
}

pub struct UpdateSearch {
    pub package_info: PackageInfo,
}

pub struct SearchPackage {
    pub query: String,
}

impl Message for UpdateSearch {
    type Result = Result<UpdateTransaction, Error>;
}

impl Message for SearchPackage {
    type Result = Result<Vec<PackageName>, Error>;
}

impl Handler<UpdateSearch> for SearchEngine {
    type Result = Result<UpdateTransaction, Error>;

    fn handle(&mut self, msg: UpdateSearch, _: &mut Self::Context) -> Self::Result {
        let mut writer = self.index.writer_with_num_threads(1, 10 * 1024 * 1024)?;

        // remove previous document of this package
        writer.delete_term(Term::from_field_text(
            self.fields.group_package,
            msg.package_info.name.as_normalized(),
        ));

        let mut document = Document::default();
        document.add_text(
            self.fields.group_package,
            msg.package_info.name.as_normalized(),
        );
        document.add_text(self.fields.group, msg.package_info.name.group());
        document.add_text(self.fields.pacakge, msg.package_info.name.name());
        if let Some(description) = msg.package_info.description {
            document.add_text(self.fields.description, &description);
        }
        for keyword in msg.package_info.keywords {
            document.add_text(self.fields.keywords, &keyword);
        }

        writer.add_document(document);

        let transaction = UpdateTransaction::new(writer);

        Ok(transaction)
    }
}

impl Handler<SearchPackage> for SearchEngine {
    type Result = Result<Vec<PackageName>, Error>;

    fn handle(&mut self, msg: SearchPackage, _: &mut Self::Context) -> Self::Result {
        let searcher = self.reader.searcher();

        let query_words: Vec<&str> = msg.query.split_whitespace().collect();

        let query_terms: Vec<_> = query_words
            .into_iter()
            .flat_map(|word| -> Vec<Box<dyn Query>> {
                vec![
                    Box::new(FuzzyTermQuery::new(
                        Term::from_field_text(self.fields.group, word),
                        // tantivy now restricts the lev length to 0 or 1,
                        // but now we are using a forked version which lifts this
                        // to 4 or 5.
                        5,
                        true,
                    )),
                    Box::new(FuzzyTermQuery::new(
                        Term::from_field_text(self.fields.pacakge, word),
                        5,
                        true,
                    )),
                    Box::new(TermQuery::new(
                        Term::from_field_text(self.fields.keywords, word),
                        IndexRecordOption::Basic,
                    )),
                    Box::new(TermQuery::new(
                        Term::from_field_text(self.fields.description, word),
                        IndexRecordOption::Basic,
                    )),
                ]
            }).map(|term| (Occur::Should, term))
            .collect();

        let query = BooleanQuery::from(query_terms);

        let top_docs: Vec<(Score, DocAddress)> =
            searcher.search(&query, &TopDocs::with_limit(50))?;

        let results = top_docs
            .into_iter()
            .filter_map(|(_score, doc_address)| -> Option<PackageName> {
                let retrieved_doc = searcher.doc(doc_address).ok()?;
                let group = retrieved_doc.get_first(self.fields.group)?.text()?;
                let name = retrieved_doc.get_first(self.fields.pacakge)?.text()?;
                let package_name = PackageName::new(group.to_string(), name.to_string()).ok()?;
                Some(package_name)
            }).collect();

        Ok(results)
    }
}

pub struct UpdateTransaction {
    writer: IndexWriter,
}

impl UpdateTransaction {
    pub fn new(writer: IndexWriter) -> Self {
        UpdateTransaction { writer }
    }

    pub fn commit(mut self) -> Result<(), Error> {
        self.writer.commit()?;
        Ok(())
    }
}
