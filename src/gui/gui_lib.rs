use ncurses::*;

const INSTRUCTIONS: &str = "Press F1 to exit. Press 'g' to goto. Press 'm' to make a message.\nPress 'c' to clear. Press 'r' to resize the C U B E.";

pub fn create_win(start_y: i32,
                  start_x: i32,
                  window_width: i32,
                  window_height: i32) -> WINDOW {
    let win = newwin(window_height, window_width, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win // *-ptr to return
}

pub fn destroy_win(win: WINDOW) {
    let ch = ' ' as chtype;
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(win);
    delwin(win);
}

pub fn put_pos(start_y: i32, start_x: i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    /* Get the screen bounds. */
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    mv(LINES() - 4, 0);
    for _i in 0..max_x {
        addstr("-");
    }
    mv(LINES() - 3, 0);
    clrtoeol();
    attron(A_BOLD());
    mvprintw(LINES() - 3, 0, format!("X: {} Y: {}", start_y, start_x).as_str());
    attroff(A_BOLD());
    mv(LINES() - 2, 0);
    clrtoeol();
    mv(LINES() - 1, 0);
    clrtoeol();
    mvprintw(LINES() - 2, 0, &INSTRUCTIONS);
}

pub fn put_alert(x_loc: i32,
                 y_loc: i32,
                 x_dim: i32,
                 y_dim: i32,
                 message: &str) {
    let mut max_x = 0;
    let mut max_y = 0;
    let start_x;
    let start_y;

    match x_loc+y_loc {
        -2 => {
            /* Get the screen bounds. */
            getmaxyx(stdscr(), &mut max_y, &mut max_x);
            start_y = max_y / 2;
            start_x = max_x / 2;
        },
        _ => {
            max_x = x_loc;
            max_y = y_loc;
            start_y = max_y;
            start_x = max_x;
        },
    }

    let win = newwin((y_dim)+2, (x_dim)+2, start_y, start_x);
    if message.len() > (x_dim as usize) {
        let real_x_dim = x_dim as usize;
        for i in 0..message.len(){
            let i_i32 = i as i32;
            if i == 0 {
                mvprintw(start_y+1+i_i32, start_x+1, &message[0..real_x_dim]);
            } else if real_x_dim*(i+1) >= message.len() {
                mvprintw(start_y+1+i_i32, start_x+1, &message[real_x_dim*(i)..]);
                break;
            } else {
                mvprintw(start_y+1+i_i32, start_x+1, &message[real_x_dim*(i)..real_x_dim*(i+1)]);
            }
        }
    } else {
        mvprintw(start_y+1, start_x+1, &message);
    }
    box_(win, 0, 0);
    wrefresh(win);
}
