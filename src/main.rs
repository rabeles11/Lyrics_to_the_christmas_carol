fn main() {
    let lyrics = ["A partridge in a pear tree"
    ,"Two turtle doves","Three French hens","Four calling birds","Five golden rings (five golden rings)",
    "Six geese a laying","Seven swans a swimming","Eight maids a milking","Nine ladies dancing",
    "Ten lords a leaping","I sent 11 pipers piping","12 drummers drumming"];

    for index in 1..13{
        println!("On the {} day of Christmas,\nMy true love sent to me",index);
        for index_rev in (0..index).rev(){
            println!("{}",lyrics[index_rev])
        }
        println!("\n");
    }
}
