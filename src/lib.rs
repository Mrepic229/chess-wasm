
use owlchess::{board, moves, Board, Make, Move};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn next_move(chess_move: &str, board: &str) -> String {
    //log(&format!("FEN: {}", board));
    let imported_board = Board::from_fen(&board);
    let result_imported_board = match imported_board {
        Ok(v) => v,
        Err(e) => {
            return format!("cannot parse FEN");
        }
    };
    let new_move = Move::from_san(&chess_move, &result_imported_board);
    let result_new_move = match new_move {
        Ok(v) => v,
        Err(e) => {
            return format!("cannot parse san");
        }
    };
    let new_result_move = result_new_move.make(&result_imported_board);
    let result_new_result_move = match new_result_move {
        Ok(v) => v,
        Err(e) => {
            return format!("cannot simulate move");
        }
    };

    let mut gameover = "";
    if !&result_new_result_move.has_legal_moves(){
        if result_new_result_move.is_check() {
            gameover ="checkmate";
        } else{
            gameover = "stalemate";
        }
    }
    
    return format!("{}{}", &result_new_result_move, gameover);
}