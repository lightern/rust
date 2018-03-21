use std::process::exit;


fn rivitesti(n: usize, indeksi: usize, sudoku: &[usize]) -> usize {
let rivi = match indeksi {
                0...8 => 0,
                9...17 => 1,
                18...26 => 2,
                27...35 => 3,
                36...44 => 4,
                45...53 => 5,
                54...62 => 6,
                63...71 => 7,
                _ => 8,
                };
                
                // tsekkaa rivin solut
                let mut counter = 0;
                let result = loop {
                    if rivi * 9 + counter != indeksi && sudoku[rivi*9+counter] == sudoku[indeksi] {
                        break 0;
                    }
                    if counter == 8 {
                        break 1;
                    }
                    counter += 1;
                };
                result
}

fn saraketesti(n: usize, indeksi: usize, sudoku: &[usize]) -> usize {
let sarake = match indeksi {
                0 | 9 | 18 | 27 | 36 | 45 | 44 | 63 | 72 => 0,
                1 | 10 | 19 | 28 | 37 | 46 | 55 | 64 | 73 => 1,
                2 | 11 | 20 | 29 | 38 | 47 | 56 | 65 | 74 => 2,
                3 | 12 | 21 | 30 | 39 | 48 | 57 | 66 | 75 => 3,
                4 | 13 | 22 | 31 | 40 | 49 | 58 | 67 | 76 => 4,
                5 | 14 | 23 | 32 | 41 | 50 | 59 | 68 | 77 => 5,
                6 | 15 | 24 | 33 | 42 | 51 | 60 | 69 | 78 => 6,
                7 | 16 | 25 | 34 | 43 | 52 | 61 | 70 | 79 => 7,
                _ => 8,
                };

                // tsekataan sarakkeen solut
                let mut counter = 0;
                let result = loop {
                    if counter * 9 + sarake != indeksi && sudoku[counter * 9 + sarake] == sudoku[indeksi] {
                        //println!("Sudoku indeksi: {}.", indeksi + 1);
                        //println!("Sarake: {}, rivi: {}.", sarake + 1, counter + 1);
                        break 0;
                    }
                    if counter == 8 {
                        break 1;
                    }
                    counter += 1;
                };
                result

}

fn laatikkotesti(indeksi: usize, sudoku: &[usize]) -> usize {
let laatikko = match indeksi {
                    0 | 1 | 2 | 9 | 10 | 11 | 18 | 19 | 20 => 0,
                    3 | 4 | 5 | 12 |13 | 14 | 21 | 22 | 23 => 1,
                    6 | 7 | 8 | 15 | 16 | 17 | 24 | 25 | 26 => 2,
                    27 | 28 | 29 | 36 | 37 | 38 | 45 | 46 | 47 => 3,
                    30 | 31 | 32 | 39 | 40 | 41 | 48 | 49 | 50 => 4,
                    33 | 34 | 35 | 42 | 43 | 44 | 51 | 52 | 53 => 5,
                    54 | 55 | 56 | 63 | 64 | 65 | 72 | 73 | 74 => 6,
                    57 | 58 | 59 | 66 | 67 | 68 | 75 | 76 | 77 => 7,
                    _ => 8,
                    };

                    let laatikkoarvot = match laatikko {
                        0 => vec![0, 1, 2, 9, 10, 11, 18, 19, 20],
                        1 => vec![3, 4, 5, 12, 13, 14, 21, 22, 23],
                        2 => vec![6, 7, 8, 15, 16, 17, 24, 25, 26],
                        3 => vec![27, 28, 29, 36, 37, 38, 45, 46, 47],
                        4 => vec![30, 31, 32, 39, 40, 41, 48, 49, 50],
                        5 => vec![33, 34, 35, 42, 43, 44, 51, 52, 53],
                        6 => vec![54, 55, 56, 63, 64, 65, 72, 73, 74],
                        7 => vec![57, 58, 59, 66, 67, 68, 75, 76, 77],
                        _ => vec![60, 61, 62, 69, 70, 71, 78, 79, 80],
                    };
                    
                    let mut counter = 0;
                    let result = loop {
                        if laatikkoarvot[counter] != indeksi && sudoku[laatikkoarvot[counter]] == sudoku[indeksi] {
                            break 0;
                        }
                        if counter == 8 {
                            break 1;
                        }
                        counter += 1;
                    };
                    result

}

fn main() {
    let mut sudoku = vec![8,0,3,0,4,0,0,7,5,4,0,0,3,0,0,0,6,8,0,0,0,9,5,0,0,0,3,5,0,0,0,0,2,7,0,0,6,0,2,8,0,4,0,0,9,0,0,9,0,7,0,4,3,0,7,0,8,0,0,1,0,0,0,3,0,0,0,0,0,0,0,7,0,4,6,0,2,0,0,0,0];
    let mut solvables_index = vec![];
    
    // Let's create index list for those that are solvable
    for i in 0..sudoku.len() {
        if sudoku[i] == 0 {
            solvables_index.push(i);
        }
    }
    
    // n represents an index in whole sudoku number set
    let mut n: usize = 0;

    'seuraava: loop {
        'numerohaku: loop {
            
            // In case of drop (coming backward from higher solvable_index)
            if sudoku[solvables_index[n]] == 9
                    {
                        sudoku[solvables_index[n]] = 0;
                        n -= 1;
                        continue 'numerohaku;
                    }

            sudoku[solvables_index[n]] += 1;
            let mut testisum = 0;

            // Check row
            testisum += rivitesti(n, solvables_index[n], &sudoku);
            if testisum == 1 {
                // Check column
                testisum += saraketesti(n, solvables_index[n], &sudoku);}
                    if testisum == 2 {
                        // Check "box"
                        testisum += laatikkotesti(solvables_index[n], &sudoku);}
            
            if testisum < 3 { 
                if sudoku[solvables_index[n]] == 9
                    {
                        sudoku[solvables_index[n]] = 0;
                        if n == 0 {
                            println!("Answer not found!");
                            exit(666);
                            }
                        
                        n -= 1;
                    }
                continue 'numerohaku;
            }

            if n == solvables_index.len()-1 {
                break 'seuraava;
            }

            n += 1;
        }
    }
    
    println!("");
    for x in 0..9
    {
        println!("  {} {} {} | {} {} {} | {} {} {}", sudoku[0+x*9], sudoku[1+x*9], sudoku[2+x*9], sudoku[3+x*9], sudoku[4+x*9], sudoku[5+x*9], sudoku[6+x*9], sudoku[7+x*9], sudoku[8+x*9]);
        if x == 2 || x == 5
        {
            println!("  ---------------------");
        }
    }
    println!("");
}
