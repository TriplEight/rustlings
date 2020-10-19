// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

// Put your function here!
// fn calculate_apple_price(_apple :i32) -> i32 {
//     let _price :i32;

//     if _apple > 40 {
//         let price = _apple;
//         price
//     } else {
//         let price = _apple * 2;
//         price
//     }
// } // QUESTION

fn calculate_apple_price(apple :i32) -> i32 {
    let price :i32;

    if apple > 40 {
        price = apple; // QUESTION
        price // QUESTION
    } else {
        price = apple * 2;
        price
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
