use std::io;
use genpdf::{elements};
 
fn main() {
    // Variables to store user input as strings
    
    let mut stu_name=String::new(); 
    let mut total_marks=String::new();  
    let mut no_of_subjects=String::new(); 
 

    println!("Enter the Student Name");
        io::stdin()
        .read_line(&mut stu_name)
        .expect("Failed to read line");

    println!("Enter Total Marks");
        io::stdin()
        .read_line(&mut total_marks)
        .expect("Failed to read line");

    println!("Enter No of Subjects ");
        io::stdin()
        .read_line(&mut no_of_subjects)
        .expect("Failed to read line");

        let name=stu_name.trim();

        let total_marks: f32 =  total_marks.trim().parse().expect("please type total marks");
        let no_of_subjects: u32 = no_of_subjects.trim().parse().expect("please type no.of subjects");
        
        let average=calculate_average(total_marks,no_of_subjects);
        let grade=get_grade(average);
        
        println!("student name: {} \nmarks: {}  \nsubjects: {} \nAverage: {} \nGrade: {}", name,total_marks,no_of_subjects,average,grade);


//PDF Generating

        // Load a font from the file system
        let font_family = genpdf::fonts::from_files("./fonts/Roboto/static", "Roboto", None)
                .expect("Failed to load font family");
        // Create a document and set the default font family
        let mut doc = genpdf::Document::new(font_family);
        // Change the default settings
        doc.set_title("Student Report Card");
        // Customize the pages
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_page_decorator(decorator);
        // Add one or more elements
        let mut card = elements::LinearLayout::vertical();
        card.push(elements::Paragraph::new("Report Card"));
        card.push(elements::Break::new(1));
        card.push(elements::Paragraph::new(format!("Name: {}", name)));
        card.push(elements::Paragraph::new(format!("Total Marks: {}", total_marks)));
        card.push(elements::Paragraph::new(format!("Subjects: {}", no_of_subjects)));
        card.push(elements::Paragraph::new(format!("Average: {:.2}", average)));
        card.push(elements::Paragraph::new(format!("Grade: {}", grade)));
        
        // Render the document and write it to a file
        doc.push(card);
        doc.render_to_file("Student_Report.pdf").expect("Failed to write PDF file");
}

//Function to calculate average marks
fn calculate_average(total_marks: f32, no_of_subjects: u32) -> f32 {
    total_marks / no_of_subjects as f32
}

// Function to assign grade based on average marks
fn get_grade(avge:f32)->char{
    if avge>=90.0{
        'A'
    }
    else if avge>=75.0 {
        'B'
    }
    else if avge>=60.0 {
        'C'
    }
    else{
        'D'
    }
}

