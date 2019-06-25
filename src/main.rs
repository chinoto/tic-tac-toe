use std::io::{self,Write};

//Over-engineering for fun...
use GameFinish::*;
enum GameFinish {
	Stalemate,
	Win
}

fn main() {
	let mut board=[' ';9];
	let mut turn=false;
	loop {
		let mark=if turn {'o'} else {'x'};
		print_board(&board);
		'ask: loop {
			print!("Choose a location for {} with the number pad: ",if turn {'o'} else {'x'});
			io::stdout().flush().unwrap();
			match read_selection() {
				Ok(x)=>{
					if board[x]!=' ' {
						println!("A mark already exists at that location.");
						continue 'ask;
					}
					board[x]=mark;
					break 'ask;
				}
				Err(x)=>{println!("{}", x);}
			}
		}
		if let Some(check)=check_game(&board,mark) {
			print_board(&board);
			match check {
				Stalemate => println!("Stalemate!"),
				Win => println!("{} wins!", mark)
			}
			std::process::exit(0);
		}
		turn=!turn;
	}
}

fn print_board(&b: &[char;9]) {
	println!("
{}|{}|{}
-+-+-
{}|{}|{}
-+-+-
{}|{}|{}",
		b[0],b[1],b[2],
		b[3],b[4],b[5],
		b[6],b[7],b[8]
	);
}

fn read_selection() -> Result<usize,&'static str> {
	let mut buffer=String::new();
	io::stdin().read_line(&mut buffer).unwrap();
	if buffer.len()!=2 {return Err("Invalid space.");}
	let mut cell=buffer.chars().next().unwrap() as usize;
	//49-58 correspond to 1-9.
	if cell<49 || cell>57 {return Err("Invalid space.");}
	cell-=49;
	Ok(cell%3+(2-cell/3)*3)
}

fn check_game(b: &[char;9], m: char) -> Option<GameFinish> {
	//0=unfinished, 1=win, 2=stalemate
	if
		b[4]==m && (b[0]==m && b[8]==m || b[2]==m && b[6]==m)
		||(0..3).any(|y| (0..3).all(|x| b[x+y*3]==m))
		||(0..3).any(|x| (0..3).all(|y| b[x+y*3]==m))
	{Some(Win)}
	else if b.iter().all(|&m| m!=' ') {Some(Stalemate)}
	else {None}
}
