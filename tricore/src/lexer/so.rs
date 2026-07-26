use crate::reader::Reader;

pub fn doc_so(reader: &mut Reader) -> String {
    let mut ket_qua = String::new();
    let mut da_gap_dau_cham = false;

    while let Some(c) = reader.peek() {
        if c.is_ascii_digit() {
            ket_qua.push(c);
            reader.next();
        } else if c == '.' && !da_gap_dau_cham {
            da_gap_dau_cham = true;
            ket_qua.push(c);
            reader.next();
        } else {
            break;
        }
    }

    ket_qua
}
