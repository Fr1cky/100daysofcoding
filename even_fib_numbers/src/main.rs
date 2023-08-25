// this really took me a while but finally figured it out.
fn main() {
    let mut initial = 0;
    let mut end = 1;
    let mut total ;
    let mut ans = 0;

    while end < 4000000 {  
        
        total = initial + end; 

        initial = end;
        end = total;

        if total % 2 ==0 { 
            ans += total;
            print!("Answer, {}", ans)
        }


        }
       
    
}
