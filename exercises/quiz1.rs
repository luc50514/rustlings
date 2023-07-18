// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought. No hints this time!
//
// No hints this time ;)

// Put your function here!
 fn calculate_price_of_apples(price:i32) -> i32 {
    if price <= 40 {
    return 2*price
    }
   return price
 }

// Don't modify this function!
#[test]
fn verify_test_for_35() {
    let price1 = calculate_price_of_apples(35);
    assert_eq!(70, price1);
}
#[test]
fn verify_test_for_40() {
    
    let price2 = calculate_price_of_apples(40);

    assert_eq!(80, price2);
}
#[test]
fn verify_test_for_41() {
   
    let price3 = calculate_price_of_apples(41);

    assert_eq!(41, price3);
}
#[test]
fn verify_test_for_65() {
   
    let price4 = calculate_price_of_apples(65);

    assert_eq!(65, price4);
}
