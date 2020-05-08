#[allow(unused_doc_comments)]
#[allow(unused_variables)]
// Single line comments look like this

/**
 * Multi line comments look like
 * this...
 */

/**
 * This is the main, function it the entry point to the program
 */
fn main() {
    println!("Hello, world!");

    // Built in types

    // Scalars

    /**
     * Signed Integers
     *
     * A refresher:
     * Normally when you think about how many numbers you can store in a certain bit size you use
     * the formula (2^num_bits) - 1. So for example you can store in 8 bits
     *
     * 2^8 -1 = 255
     *
     * This is only the case for unsigned ints. In the case of signed ints, one bit is used
     * to store the sign of the value. So we actually end up with one sign bit and n - 1 value bits
     *
     * So as can be seen below i8 does not represent 0-255 but instead -127 to 127
     */
    let signed_eight: i8 = 127; // 8 bit signed integer, represents -127 to 127
    let signed_sixteen: i16 = 32767; // 16 bit signed integer, represents -32767 to 32767
    let signed_thirty_two: i32 = 2147483647; // 32 bit signed integer, represents -2147483647 to 2147483647
    let signed_sixty_four: i64 = 9223372036854775807; // 64 bit signed integer, represents -9223372036854775807 to 9223372036854775807
    let signed_one_twenty_eight: i128 = 9223372036854775808; // 128 bit signed integer, represents -1.7014118e+38 to 1.7014118e+38

    /**
     * Unsigned Integers
     *
     * This is a more familiar type of int, It represents values from 0 -> 2^n - 1.
     * They cannot be negative
     */
    let unsigned_eight: u8 = 255; // 8 bit signed integer, represents 0 to 2^8 - 1
    let unsigned_sixteen: u16 = 32767; // 16 bit signed integer, represents 0 to 2^16 - 1
    let unsigned_thirty_two: u32 = 2147483647; // 32 bit signed integer, represents 0 to 2^32 - 1
    let unsigned_sixty_four: u64 = 9223372036854775807; // 64 bit signed integer, represents 0 to 2^64 - 1
    let unsigned_one_twenty_eight: u128 = 9223372036854775808; // 128 bit signed integer, represents 0 to 2^128 - 1

    /**
     * Sizes
     *
     * These types are determined at compile type and are capable of storing values up to one word
     * size of the target machine. For example on a 32 bit machine
     *
     * isize -> i32
     * usize -> u32
     *
     * And the same logic applies on 64 bit systems
     */
    let i_size: isize = 0;
    let u_size: usize = 0;

    /**
     * Floats
     *
     * Rust provides both 32 and 64 bit float types. You denote a float by placing a decimal place
     * on the value, duh...
     */
    let float_thirty_two: f32 = 2147483648.0;
    let float_sizty_four: f64 = 9_223_372_036_854_775_807.0;

    /**
     * You can also use the underscore in numeric literals to improve readability
     */

    /**
     * Char
     *
     * Char represents a character which takes up 4 bytes in memory
     */
    let a_char: char = 'a';

    /**
     * Bool
     *
     * Pretty self explanatory, the usual && || and ! also works with these types
     */
    let true_bool: bool = true;

    /**
     * Unit type
     *
     * This type represents an empty tuple
     */
    let empty_tuple: () = ();

    // Compound Types

    /**
     * Tuples
     *
     * A collection of values of different types
     */
    let my_tuple: (i32, bool) = (100, true);

    /**
     * Arrays
     *
     * A set of items of the same type stored continuously in memory. Their size must be known at compile
     * time. So it is provided in the type signature
     */
    let my_array: [u32; 3] = [10, 10, 10];

    // We can set all values to some default like this
    let init_arr: [i32; 100] = [10; 100];

    /**
     * Slices
     *
     * But what if the size of your array is not known at compile time?
     * Well in this case you can use a Slice, Slices are made up of two
     * components, the first one is a pointer to the data and the second
     * one is the length of the slice. To describe a slice as a type we use
     *
     * &[T]
     *
     * For example &[i32]
     *
     * So this is what we would use when we are for example passing an array of
     * unknown sixe to a function, We can also use slices to pull vales from an array
     * for this we use the syntax &VARNAME[START_INDEX..END_INDEX]
     *
     * For example
     */
    let mut slc: &[i32];
    let arr_to_slc: [i32; 10] = [5; 10];
    slc = &arr_to_slc[1..6];

    // Custom Data Types

    /**
     * Structs
     *
     * Rust does not have classes so be prepared for that. It is not traditionally
     * object oriented. Instead it has its own constructs that parralel some of the
     * ideas in object orientation
     *
     * Structs are a bit of a deep subject and there are multiple types.
     *
     * The first type of struct is a Tuple Struct
     */

    struct Point3D(i32, i32, i32);
    let point_instance: Point3D = Point3D(19, 10, 88);
    let x = point_instance.0;
    let Point3D(x, y, z) = point_instance;

    /**
     * "C" Structs
     *
     * These are probably what you are thinking of when you think of a struct
     */
    struct Person {
        age: u8,
        fav_number: u128,
    }
    let me: Person = Person {
        age: 10,
        fav_number: 1000,
    };
    let age: u8 = me.age;
}
