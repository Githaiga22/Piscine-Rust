pub fn delete_and_backspace(s: &mut String) {
    let mut res:String = String::new();
    let  st: Vec<char> = s.chars().collect();
    
    let mut i = 0;
    while  i < st.len(){
        if st[i] == '-' && !st.is_empty(){
            res.pop();
            i += 1;
        }else if st[i] == '+' {
            let mut count = 0;
           while i < st.len() && st[i] == '+'{
            count += 1;
            i += 1;
           }

            i = i + count;
        }else {
            res.push(st[i]);
            i += 1;
        }
    }
    *s = res;
}

pub fn do_operations(v: &mut [String]) {
    for n in v.iter_mut() {
        let mut r = 0;
        if n.contains("+") {
            let parts:Vec<&str> = n.split("+").collect();
            let n1 = parts[0].parse::<i32>().unwrap_or(0);
            let n2 = parts[1].parse::<i32>().unwrap_or(0);

            r = n1 + n2; 
        }else if n.contains("-") {
            let parts:Vec<&str> = n.split("-").collect();
            let n1 = parts[0].parse::<i32>().unwrap_or(0);
            let n2 = parts[1].parse::<i32>().unwrap_or(0);

            r = n1 - n2;
        };
        *n = r.to_string();
    }
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
        let mut b = [
            "2+2".to_owned(),
            "3+2".to_owned(),
            "10-3".to_owned(),
            "5+5".to_owned(),
        ];
        delete_and_backspace(&mut a);
        do_operations(&mut b);

        assert_eq!(a, "borrow");
        assert_eq!(b, ["4", "5", "7", "10"]);
    }
}