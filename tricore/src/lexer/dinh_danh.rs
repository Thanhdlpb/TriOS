use crate::reader::Reader;

pub fn doc_dinh_danh(reader:&mut Reader)->String{

    reader.doc_khi(|c|{

        c.is_alphanumeric()

        ||

        c=='_'

        ||

        c.is_alphabetic()

    })

}
