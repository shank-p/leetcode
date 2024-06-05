/*
    409. Longest Palindrome
    -> leetcode (easy)
    https://leetcode.com/problems/longest-palindrome/description/?envType=daily-question&envId=2024-06-04
*/


use std::io;

pub fn longest_palindrome(s: String) -> i32 {
    /*
    Array Counter implementation.
        
    Time Complexity : O(N) - linear | N is length of string .
    Space Complxity : O(1) - constant. 
    */

    // initialize arrays as counter for uppercase & lowercase charaters.
    let mut uppercase_map: [u8; 26] = [0; 26];
    let mut lowercase_map: [u8; 26] = [0; 26];
    for c in s.chars(){
        let idx: usize = c as usize;
        if idx <= 91{
            uppercase_map[idx-65] += 1;
        } else {
            lowercase_map[idx-97] += 1;
        }
    }

    let mut odd_count_length: i32 = 0;

    for idx in 0..26 {
        if uppercase_map[idx]%2 != 0 {
            odd_count_length += 1;
        }
        if lowercase_map[idx]%2 != 0 {
            odd_count_length += 1;
        }
    }
    if odd_count_length == 0 {
        return s.len() as i32;
    }
    s.len() as i32 - odd_count_length + 1

}


fn main() {
    let mut s: String = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Unable to read input for `s`!");
    let s: String = s.trim().to_string();

    println!("-->> result : {}", longest_palindrome(s));
}
