pub fn foo() {
    pkg1::hello()
}

pub trait MetadataGenerator {
    fn generate_metadata(&self, v: &dyn pkg1::MetadataVisitor);
}

//---

pub struct CacheGenerator;

impl MetadataGenerator for CacheGenerator {
    fn generate_metadata(&self, v: &dyn pkg1::MetadataVisitor) {
        println!("CacheGenerator.generate_metadata()");
        v.superblock_b();
    }
}

//---

pub fn accept_metadata_visitor(v: &dyn pkg1::MetadataVisitor) {
    v.superblock_b();
}
