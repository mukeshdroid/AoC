/// produces a number concatenating the first and last digit in a alphanumeric string
/// ```
/// use AoC_2023::extract_num;
/// assert_eq!(extract_num(String::from("hjasd1hgsdhja2ghs31g36a")),16);
/// ```

pub fn extract_num(s : String) -> u32{
    let mut nums: Vec<u32> = Vec::new();
    for c in s.chars(){
        if c.is_numeric(){
            nums.push(c.to_digit(10).unwrap());
        }
    }
    nums.get(0).unwrap() * 10 + nums.last().unwrap()
}