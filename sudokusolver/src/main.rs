extern crate gtk; // GUI
use gtk::prelude::*; // GUI


// use std::process::exit;  // In case you want to make it without GUI.


// make moving clones into closures more convenient
    macro_rules! clone {
        (@param _) => ( _ );
        (@param $x:ident) => ( $x );
        ($($n:ident),+ => move || $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move || $body
            }
        );
        ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
            }
        );
}


fn rowtest(indexnumber: usize, sudoku: &[usize]) -> usize {
    let row = match indexnumber {
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

    let mut counter = 0;
    let result = loop {
        if row * 9 + counter != indexnumber && sudoku[row * 9 + counter] == sudoku[indexnumber] {
            break 0;
        }
        if counter == 8 {
            break 1;
        }
        counter += 1;
    };
    result
}

fn columntest(indexnumber: usize, sudoku: &[usize]) -> usize {
    let column = match indexnumber {
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

    let mut counter = 0;
    let result = loop {
        if counter * 9 + column != indexnumber
            && sudoku[counter * 9 + column] == sudoku[indexnumber]
        {
            break 0;
        }
        if counter == 8 {
            break 1;
        }
        counter += 1;
    };
    result
}

fn boxtest(indexnumber: usize, sudoku: &[usize]) -> usize {
    let boxnumber = match indexnumber {
        0 | 1 | 2 | 9 | 10 | 11 | 18 | 19 | 20 => 0,
        3 | 4 | 5 | 12 | 13 | 14 | 21 | 22 | 23 => 1,
        6 | 7 | 8 | 15 | 16 | 17 | 24 | 25 | 26 => 2,
        27 | 28 | 29 | 36 | 37 | 38 | 45 | 46 | 47 => 3,
        30 | 31 | 32 | 39 | 40 | 41 | 48 | 49 | 50 => 4,
        33 | 34 | 35 | 42 | 43 | 44 | 51 | 52 | 53 => 5,
        54 | 55 | 56 | 63 | 64 | 65 | 72 | 73 | 74 => 6,
        57 | 58 | 59 | 66 | 67 | 68 | 75 | 76 | 77 => 7,
        _ => 8,
    };

    let boxvalues = match boxnumber {
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
        if boxvalues[counter] != indexnumber && sudoku[boxvalues[counter]] == sudoku[indexnumber] {
            break 0;
        }
        if counter == 8 {
            break 1;
        }
        counter += 1;
    };
    result
}

fn solve(sudoku: &mut Vec<usize>) -> (&mut Vec<usize>) {
    let mut solvables_index = vec![];

    // Let's create list of indexes for those that need to be solved
    for i in 0..sudoku.len() {
        if sudoku[i] == 0 {
            solvables_index.push(i);
        }
    }

    // n represents an index in whole sudoku number set
    let mut n: usize = 0;

    'nextn: loop {
        'numbersearch: loop {
            // In case of drop (coming backward from higher solvable_index)
            if sudoku[solvables_index[n]] == 9 {
                sudoku[solvables_index[n]] = 0;
                n -= 1;
                continue 'numbersearch;
            }

            sudoku[solvables_index[n]] += 1;
            let mut testsum = 0;

            // Check row
            testsum += rowtest(solvables_index[n], &sudoku);
            if testsum == 1 {
                // Check column
                testsum += columntest(solvables_index[n], &sudoku);
            }
            if testsum == 2 {
                // Check "box"
                testsum += boxtest(solvables_index[n], &sudoku);
            }

            if testsum < 3 {
                if sudoku[solvables_index[n]] == 9 {
                    sudoku[solvables_index[n]] = 0;
                    if n == 0 {
                        println!("Answer not found!");
                        // exit(666);          // In case you want to make it without GUI
                        return sudoku;
                    }

                    n -= 1;
                }
                continue 'numbersearch;
            }

            if n == solvables_index.len() - 1 {
                break 'nextn;
            }

            n += 1;
        }
    }

    println!("");
    for x in 0..9 {
        println!(
            "  {} {} {} | {} {} {} | {} {} {}",
            sudoku[0 + x * 9],
            sudoku[1 + x * 9],
            sudoku[2 + x * 9],
            sudoku[3 + x * 9],
            sudoku[4 + x * 9],
            sudoku[5 + x * 9],
            sudoku[6 + x * 9],
            sudoku[7 + x * 9],
            sudoku[8 + x * 9]
        );
        if x == 2 || x == 5 {
            println!("  ---------------------");
        }
    }
    println!("");
    sudoku
}

pub fn launch() {
    gtk::init().unwrap_or_else(|_| panic!("panic!"));

    let builder = gtk::Builder::new_from_string(include_str!("app_window.ui"));
    let window: gtk::Window = builder.get_object("Window1").unwrap();
    let bigbutton: gtk::Button = builder.get_object("button1").expect("Couldn't get button1");
    let dialog: gtk::Dialog = builder.get_object("Dialog1").expect("Couldn't get dialog");
    let smallbutton: gtk::Button = builder.get_object("Button2").expect("Couldn't get button2");
    let textbox: gtk::TextView = builder.get_object("Textbox1").expect("Couldn't get dialog");
    let buffer: gtk::TextBuffer = builder.get_object("Buffer1").expect("Couldn't get buffer");
    //g_signal_connect_swapped(dialog, "response", G_CALLBACK (gtk_widget_destroy), dialog);

    // window.connect_delete_event(clone!(window => move |_, _| {

    bigbutton.connect_clicked(move |_| {
        let mut sudoku = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        for n in 1..sudoku.len() + 1 {
            let mut nameref = String::from("entry");
            nameref.push_str(&n.to_string());
            let entry: gtk::Entry = builder.get_object(&nameref).expect(&nameref);
            if entry.get_text().unwrap() != "" {
                sudoku[n - 1] = entry.get_text().unwrap().parse::<usize>().unwrap();
            }
        }

        let vastaus: &mut Vec<usize> = solve(&mut sudoku);
        // let mut vastausstring = String::new();
        // for x in 0..9 {
        //     vastausstring.push(x.)
        // }
        dialog.show_all();

        // let textimport: String = "lollerball".to_string();
        //buffer.set_text(&textimport);
        buffer.set_text(&format!(
"{}   {}   {}   |   {}   {}   {}   |   {}   {}   {}
{}   {}   {}   |   {}   {}   {}   |   {}   {}   {}
{}   {}   {}   |   {}   {}   {}   |   {}   {}   {}
------------------------------
{}   {}   {}   |   {}   {}   {}   |   {}   {}   {}
{}   {}   {}   |   {}   {}   {}   |   {}   {}   {}
{}   {}   {}   |   {}   {}   {}   |   {}   {}   {}
------------------------------
{}   {}   {}   |   {}   {}   {}   |   {}   {}   {}
{}   {}   {}   |   {}   {}   {}   |   {}   {}   {}
{}   {}   {}   |   {}   {}   {}   |   {}   {}   {}"
            , vastaus[0], vastaus[1], vastaus[2], vastaus[3], vastaus[4], vastaus[5], vastaus[6], vastaus[7], vastaus[8]
            , vastaus[9], vastaus[10], vastaus[11], vastaus[12], vastaus[13], vastaus[14], vastaus[15], vastaus[16], vastaus[17]
            , vastaus[18], vastaus[19], vastaus[20], vastaus[21], vastaus[22], vastaus[23], vastaus[24], vastaus[25], vastaus[26]
            , vastaus[27], vastaus[28], vastaus[29], vastaus[30], vastaus[31], vastaus[32], vastaus[33], vastaus[34], vastaus[35]
            , vastaus[36], vastaus[37], vastaus[38], vastaus[39], vastaus[40], vastaus[41], vastaus[42], vastaus[43], vastaus[44]
            , vastaus[45], vastaus[46], vastaus[47], vastaus[48], vastaus[49], vastaus[50], vastaus[51], vastaus[52], vastaus[53]
            , vastaus[54], vastaus[55], vastaus[56], vastaus[57], vastaus[58], vastaus[59], vastaus[60], vastaus[61], vastaus[62]
            , vastaus[63], vastaus[64], vastaus[65], vastaus[66], vastaus[67], vastaus[68], vastaus[69], vastaus[70], vastaus[71]
            , vastaus[72], vastaus[73], vastaus[74], vastaus[75], vastaus[76], vastaus[77], vastaus[78], vastaus[79], vastaus[80]
            ));

        
    });

    // window.connect_delete_event(clone!(window => move |_, _| {
    //smallbutton.connect_clicked(move |_y| {dialog.destroy()});


    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

fn main() {
    launch();
}
