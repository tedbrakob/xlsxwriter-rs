use xlsxwriter::{
    worksheet::conditional_format::{ConditionalDataBar, ConditionalFormat},
    Workbook,
};

fn main() -> Result<(), xlsxwriter::XlsxError> {
    let workbook = Workbook::new("target/databars.xlsx")?;

    let mut sheet1 = workbook.add_worksheet(None)?;

    sheet1.write_number(0, 0, 20., None)?;
    sheet1.write_number(1, 0, 21., None)?;
    sheet1.write_number(2, 0, 22., None)?;
    sheet1.write_number(3, 0, 23., None)?;
    sheet1.write_number(4, 0, 24., None)?;
    sheet1.write_number(5, 0, 25., None)?;
    sheet1.write_number(6, 0, 26., None)?;
    sheet1.write_number(7, 0, 27., None)?;
    sheet1.write_number(8, 0, 28., None)?;

    sheet1.conditional_format_range(
        0,
        0,
        8,
        0,
        &ConditionalFormat::DataBar(ConditionalDataBar::new()),
    )?;

    workbook.close()
}
