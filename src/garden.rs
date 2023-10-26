mod garden2;
pub fn gfn1(){
    for ch1 in ('Z'..='a').rev(){
        print!("{ch1}");
    }
    println!("");
    crate::garden::garden2::gfn2();
}
