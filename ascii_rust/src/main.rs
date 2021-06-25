use std::*;
use std::fs::*;
use std::io::*;


fn matrixfeltolt<T: Clone>(lista: &mut Vec<Vec<T>>, magassag: i8 ,szelesseg: i8, toltoelem: T)
{
    
    for i in 0..magassag
    {
        lista.push(Vec::new());
        for _j in 0..szelesseg
        {
            lista[i as usize].push(toltoelem.clone());
        }   
    }
}

fn main() {
    let l: i8 = 4;
    let h: i8 = 5;
    let mut betuk: Vec<String> = Vec::new();
    println!("Írd be a szövegedet!");
    let mut t: String = String::new();
    io::stdin().read_line(&mut t).expect("Nem megfelelő szöveg");
    let file = File::open("alphabet.txt").unwrap();
    let reader = BufReader::new(file);
    
    for sor in reader.lines()
    {
        let sor = sor.unwrap();
        let mut k = l;
        let mut j = 0;
        while j < sor.chars().count() 
        {
            let substring = sor[j..k as usize].to_string();
            betuk.push(substring);
            j += l as usize;
            k += l;
        }
    }

    let mut betukmatrix: Vec<Vec<String>> = Vec::new();
    matrixfeltolt(&mut betukmatrix, h, 28, "".to_string());
    

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    while a < h
    {
        while b < 28
        {
            betukmatrix[a as usize][b] = betuk[c as usize].clone();
            b+=1;
            c+=1;
        }
        a+=1;
        b=0;
    }

    let abc: [char; 28] =  ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',' ','?'];
    let bemenet: Vec<char> = t.trim().to_uppercase().chars().collect();
    let mut indexek: Vec<i8> = Vec::new();
    let mut nincs: i8 = 0;
    let mut bem: usize = 0;
    let mut chars: usize = 0;
    while bem < bemenet.iter().count()
    {
        while chars < abc.iter().count()
        {
            nincs+=1;
            if abc[chars] == bemenet[bem]
            {
                indexek.push(chars as i8);
                nincs = 0;
            }
            else if nincs == abc.iter().count() as i8
            {
                indexek.push(27);
            }
            chars+=1;
        }
        
        chars = 0;
        bem += 1;
        nincs = 0;

    }

    let mut answer: Vec<Vec<String>> = Vec::new();
    matrixfeltolt(&mut answer,h, indexek.iter().count() as i8,"".to_string());
    
    
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < h as usize
    {
        while j < indexek.iter().count()
        {
            answer[i][j] = betukmatrix[i][indexek[j] as usize].clone();
            j+=1;
        }
        j = 0;
        i+=1;
    }

    //println!("{:?}",answer);
    for i in 0..h as usize
    {
        for j in 0..indexek.iter().count()
        {
            print!("{}",answer[i][j]);
        }
        println!();
    }
}

