#📘 Student Report Card Generator (Rust)

A simple Rust-based console application that:

✅ Collects a student's name, total marks, and number of subjects  
✅ Calculates the average  
✅ Assigns a grade based on the average  
✅ Generates a neat **PDF report card**

## Features

- Command-line user input (student data)
- Custom methods for:
  - Average calculation
  - Grade assignment (A/B/C/D)
- PDF generation using the [`genpdf`](https://crates.io/crates/genpdf) crate
- Custom fonts supported (Oswald, LiberationSans, Roboto etc.)


## Grading Logic

| Average Marks | Grade |
|---------------|--------|
| 90 and above  | A      |
| 75–89         | B      |
| 60–74         | C      |
| Below 60      | D      |


## How to Run

##1.Clone the Repository
```bash
git clone https://github.com/your-username/student-report-rust.git
cd student-report-rust

## 2.Run the Program

-cargo build
-cargo run

## 3. Output
-The console displays the student's average and grade

-A PDF file named report_card.pdf is generated in the project root
     -- Report Card

        Name: Sam
        Total Marks: 450
        Subjects: 5
        Average: 90.00
        Grade: A

