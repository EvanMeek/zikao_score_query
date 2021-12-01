use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};
use zikao_score_query::student::Student;

fn example() -> Result<(), Error> {
    let path = format!("{}/score.xlsx", env!("CARGO_MANIFEST_DIR"));
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let range = workbook
        .worksheet_range("Sheet")
        .ok_or(Error::Msg("Cannot find 'Sheet'"))??;

    let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;
    let mut stus: Vec<Student> = Vec::new();
    for col in iter {
        let (index, name, id, stu_id): (String, String, String, String) = col?;
        stus.push(Student::new(index, name, id, stu_id));
    }
    for stu in stus {
        println!("{:?}", stu);
    }
    Ok(())
}
fn main() {
    example().unwrap();
}
