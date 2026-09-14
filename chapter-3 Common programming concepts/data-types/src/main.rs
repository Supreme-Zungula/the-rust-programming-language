fn main() {
    /*
     * Numeric operations
     */
    let sum = 5 + 20;
    println!("The sum is: {sum}");

    let difference = 95.5 - 4.5;
    println!("The difference is: {difference}");

    let product = 10 * 4;
    println!("The product is: {product}");

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // result is -1;
    println!("The quotient is: {quotient} and the truncated is: {truncated}");

    let remainder = 43 % 5;
    println!("The remainder is: {remainder}");

    /*
     * The character type
     */
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("The heart eyed cat: {heart_eyed_cat}");

    /*
     * Compound Types
     */
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let first = arr[0];
    let second = arr[1];
    println!("The first element in the array is: {first}");
    println!("The second element in the array is: {second}");
}
