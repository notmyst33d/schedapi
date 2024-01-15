# Schedule API
Open source API for college schedules

## Docs
`cargo run` and go to http://localhost:10000/docs

## Reference frontend
Code for the reference frontend is located in `frontend` branch

## CSV Schedule Schema (Rust type notation)
```
day: u8 // Day that the lesson happens on
num: u8 // Number of the lesson
week_range: Range<u8> // Range of week numbers that the lesson happens on
name: String // Name of the lesson
lesson_type: Option<String> // Type of the lesson
teacher: Option<String> // Teacher initials
auditorium: String // Auditorium number or name
even: Option<bool> // "true" if lesson happens on even week numbers
odd: Option<bool> // "true" if lesson happens on odd week numbers
```
