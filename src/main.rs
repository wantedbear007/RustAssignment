
// question 01
fn check_palindrome(word: &str) -> bool {

    let letters: Vec<char> = word.chars().collect();
    let mut end: usize = letters.len() - 1;

    for i in 0..end / 2 {
        if letters[i] != letters[end] {
            println!("something not right");
            return false;
        }

        end-=1;
    } 
    true
}



// some changes
// some more changes
// question 02
fn get_first_occurrance(arr: &[isize], target: isize) -> usize {

    for x in 0..arr.len() {
        // println!("{}",arr[x]);
        if arr[x] == target {
            return x;
        }
    }
    arr.len() + 20
}

// question 3
fn get_shortest_word(sentence: &str) -> &str {
    let words: Vec<&str> =  sentence.split(" ").collect();
    let mut shortest: &str = words[0];

    for x in words.iter() {
        let size: usize = x.chars().count();

        if size < shortest.chars().count() {
            shortest = x;
        }
    }

    shortest

}

// question 04
fn check_prime(num: usize) -> bool {

    if num == 0 || num == 1 {
        return  false;
    }

    let x: usize = num;
    let mut count: usize = 0;
    for y in 1..x {
        if count > 2 {
            return false;
        }
        if x % y == 0 {
            count += 1;
            
        } 
    }

    return true;
}

// question 5
fn get_median(arr: &mut [isize]) -> f64 {
    arr.sort();


    let mid: usize = arr.len() / 2;

    println!("{}", mid);

    if mid % 2 == 0 {
        // println!(" the median is {}", arr[mid] );
        // return arr[mid - 1] + arr[mid] / 2;


       return (arr[mid - 1] + arr[mid]) as f64 / 2.0;
    } else {
        return  arr[mid] as f64; 
    }


}

// question 6

// fn common_prefix(words: Vec<&str>) {

//     let mut all_words: HashMap<u32, &str> =  HashMap::new();

//     let size = words.len();

//     for x in 0..size {

//         for y in x..size {

            
//         }
//     }
    
    
// }

// question 7
fn kth_smallest(arr:  &mut [isize], index: isize) -> isize {
    
    if arr.len() < index as usize || arr.is_empty() {
        return -1;
    }
    arr.sort();
    return arr[index as usize];    
}

// question 9
fn reverse_str(sentence: &str) -> String {
    
    let divided: Vec<char> = sentence.chars().collect();
    let mut reversed: String = String::from("");
    for x in divided.iter().rev() {
        // println!("hello {}", x);
        reversed.push(*x);
    }

    reversed
}

// question 11
fn merge_sorted_arr(arr0: &[isize], arr1: &[isize]) {
    let total_len: usize = arr0.len() + arr1.len();
    let mut merged: Vec<isize> = Vec::with_capacity(total_len);

    for x in arr0.iter() {
        merged.push(*x);
    }
    for x in arr1.iter() {
        merged.push(*x);
    }


    merged.sort();
    print!("Question 11: ");
    for x in merged.iter() {

        print!("{}, ",x );
    }
    // merged
}


// question 12
fn maximum_sub_array(arr: &[i32]) -> i32 {
    // kadane algo

    let mut sub_max = 0;
    let mut highest: i32 = 0;

    for &x in arr {
        sub_max = i32::max(0, sub_max + x);
        highest = i32::max(highest, sub_max);
    }

    highest
}

fn main() {
    println!("Hello, world!");

    // question 1 section
    let palindrome_str: String = "abbba".to_string();
    println!("Question 01: Given string {} is palindrome {}", palindrome_str, check_palindrome(&palindrome_str));

    // question 2 section
    let arr: [isize; 4] = [10, 20, 30, 40];
    println!("Question 02: Occurance index: {} if more than size {} then not found",  get_first_occurrance(&arr, 33), arr.len());

    // question 3 section
    let sentence: &str = "Hello from bhanupratap singh h";
    println!("Question 03: The shortest word in sentence '{}' is '{}'", sentence, get_shortest_word(sentence));

    // question 4 section
    let num: usize = 0;
    println!("Question 04: The given number {} is prime ? {} ", num, check_prime(num));

    // question 5 section 
    let arr: &mut [isize] = &mut [1, 2, 3, 4, 5, 6, 8, 9];
    println!("Question 05: the median of above array is {} ",  get_median(arr));

    
    // question 6 section // not implemented
    // let naa: &str = "abced";
    // let x = naa.chars().nth(1);
    // println!("{}", x);

    // question 7 section
    let nums: &mut [isize]= &mut [10, 20 ,4, 76, 21];
    println!("Question 07: the smallest number of index {} is {} ", kth_smallest(nums, 1), 1);

    // question 8 section


    // question 9 section
    let word: &str = "Hello this sentence will be reversed";
    println!("Question 08: the reversed string is '{}'", reverse_str(word));

    // question 10 is same as question 4
    // refer line number 152

    // question 11
    let arr0: &[isize] = &[10, 20, 30, 40];
    let arr1: &[isize] = &[12, 14, 15, 19];
    merge_sorted_arr(arr0, arr1);

    // question 12
    let arr3: &[i32] = &[-2, -3, 4, -1, -2];
    println!("\nQuestion 12: Maximum is {}", maximum_sub_array(arr3));



    
}


