use std::io;

fn check_win(arr: &[i32;9], p: i32) -> bool {
    if arr[0]==arr[4] && arr[0]==arr[8] && arr[0]==p {
        return true;
    }
    if arr[2]==arr[4] && arr[2]==arr[6] && arr[2]==p {
        return true;
    }
    for i in 0..3 {
        if arr[i*3]==arr[i*3+1] && arr[i*3]==arr[i*3+2] && arr[i*3]==p {
            return true;
        }
        if arr[i]==arr[i+3] && arr[i]==arr[i+6] && arr[i]==p {
            return true;
        }
    }
    return false;
}

fn check_full(a: &[i32;9]) -> bool {
    for i in 0..9 {
        if a[i]==0 {
            return false;
        }
    }
    return true;
}

fn minimax(a: &mut[i32;9], p: i32, al: i32, be: i32) -> (i32,usize) {
    let mut alpha = al;
    let mut beta = be;
    if check_win(a,-p) {
        return (-p,9);
    }
    if check_full(a) {
        return (0,9);
    }
    let mut res = -p-p;
    let mut resi : usize = 9;
    for i in 0..9 {
        if a[i]==0 {
            a[i]=p;
            let tmp = minimax(a,-p,alpha,beta).0;
            if tmp*p > res*p {
                res = tmp;
                resi=i;
            }
            a[i]=0;
            if p==-1 {
                beta = beta.min(tmp);
                if tmp <= alpha {
                    return (tmp,i);
                }
            } else {
                alpha = alpha.max(tmp);
                if tmp >= beta {
                    return (tmp,i);
                }
            }
        }
    }
    return (res,resi);
}

fn print_table(a: &[i32;9]) {
    for y in 0..3 {
        for x in 0..3 {
            if a[y*3+x]==1 {
                print!("O");
            } else if a[y*3+x]==-1 {
                print!("X");
            } else {
                print!(".");
            }
            if x<2 {
                print!("\u{2503}");
            }
        }
        if y<2 {
            print!("\n\u{2501}\u{254b}\u{2501}\u{254b}\u{2501}\n");
        }
    }
    print!("\n");
}

fn main() {
    let mut a : [i32;9] = [
        0,0,0,
        0,0,0,
        0,0,0
    ];
    let mut p = 1;
    let com = -1;
    while check_full(&a)==false && check_win(&a,-p)==false {
        print_table(&a);
        if p==com {
            print!("computer's turn:\n");
        } else {
            print!("yout turn:\n");
        }
        if p==com {
            a[minimax(&mut a,p,-2,2).1]=p;
        } else {
            loop {
                let mut txt = String::new();
                io::stdin().read_line(&mut txt).expect("failed to read from stdin");
                let mov = txt.trim().parse::<usize>().unwrap();
                if mov<9 && a[mov]==0 {
                    a[mov]=p;
                    break;
                }
                print!("no cheating!\n");
            }
        }
        p=-p;
    }
    print_table(&a);
}
