import init, { next_move } from "../pkg/chess_web_app.js";
var board_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
var link_to_image = "https://mutsuntsai.github.io/fen-tool/gen/?fen="

function do_move() {
    const moveId = document.getElementById("chess_notation");
    const boardImageId = document.getElementById("board_image");

    let new_board = next_move(String(moveId.value), board_fen);

    board_fen = new_board

    boardImageId.src = link_to_image.concat(board_fen);
    
    console.log(new_board);
}

init().then(() => {

    console.log(board_fen);
    window.do_move = do_move;
});


