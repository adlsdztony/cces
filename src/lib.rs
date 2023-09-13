use scraper::{Html, Selector};
use std::collections::HashMap;

pub fn get_cc_dataset() -> Result<HashMap<String, HashMap<String, HashMap<String, String>>>, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://sweb.hku.hk/ccacad/ccc_appl/enrol_stat.html")?;
    let body = resp.text()?;

    let fragment = Html::parse_document(&body);
    let table_selector = Selector::parse("table").unwrap();

    let mut dataset: HashMap<String, HashMap<String, HashMap<String, String>>> = HashMap::new();
    dataset.insert("First Semester".to_string(), HashMap::new());
    dataset.insert("Second Semester".to_string(), HashMap::new());

    let mut semester = "First Semester";

    for table in fragment.select(&table_selector) {
        let row_selector = Selector::parse("tr").unwrap();
        for row in table.select(&row_selector).skip(3) {
            let ds_selector = Selector::parse("td").unwrap();
            let ds: Vec<_> = row.select(&ds_selector).collect();
            if ds.len() < 6 {
                if ds[0].inner_html().contains("Second Semester")  {
                    semester = "Second Semester";
                }
                continue;
            }
            let course_list: Vec<_> = ds.iter().map(|d| {
                let font_selector = Selector::parse("font").unwrap();
                let font = d.select(&font_selector).next().unwrap();
                font.inner_html()
            }
            ).collect();
            let mut course = HashMap::new();
            course.insert("CourseCode".to_string(), course_list[0].clone());
            course.insert("CourseName".to_string(), course_list[1].clone());
            course.insert("SubClass".to_string(), course_list[2].clone());
            course.insert("Quota".to_string(), course_list[3].clone());
            course.insert("Available".to_string(), course_list[4].clone());
            course.insert("Waitlist".to_string(), course_list[5].clone());
            course.insert("Semester".to_string(), semester.to_string());
            dataset.get_mut(semester).unwrap().insert(course_list[0].clone(), course);
        }
    }
    Ok(dataset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cc_dataset() {
        let dataset = get_cc_dataset().unwrap();
        // print dataset with course code
        for (semester, courses) in dataset.iter() {
            println!("{}:", semester);
            for (course_code, course) in courses.iter() {
                println!("{}:", course_code);
                for (key, value) in course.iter() {
                    println!("{}: {}", key, value);
                }
                println!("");
            }
            println!("");
            println!("");
        }
    }
}