use std::fs;

#[allow(dead_code)]
pub fn read_input(filename: &str) -> String {
    fs::read_to_string(format!("./inputs/{}.txt", filename))
        .expect("Something went wrong reading the file")
}

#[allow(dead_code)]
pub fn assert_array_eq(arr1 : &[i32], arr2: &[i32]) {
   assert_eq!(arr1.len(), arr2.len());

    for (index, item) in arr1.iter().enumerate() {
        assert_eq!(item, &arr2[index]);
    }
}
