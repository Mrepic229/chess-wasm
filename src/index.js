import init, { next_move } from "../pkg/chess_web_app.js";

var board_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
var link_to_image = "https://mutsuntsai.github.io/fen-tool/gen/?fen=";
var alt_link_to_image = "http://www.fen-to-image.com/image/64/single/coords/";
var is_white_to_move = true;
var moveId = document.getElementById("chess_notation");

function do_move() {
    let new_board_fen = next_move(String(moveId.value), board_fen);

    if (new_board_fen == "cannot parse FEN") {
        fen_error();
        return;
    }
    if (new_board_fen == "cannot parse san") {
        san_error();
        return;
    }
    if (new_board_fen == "cannot simulate move") {
        wft_error();
        return;
    }

    switch_whose_turn();
    display(new_board_fen);
}

function display(new_board_fen) {
    const boardImageId = document.getElementById("board_image");

    board_fen = new_board_fen;

    console.log(link_to_image);
    boardImageId.src = link_to_image.concat(board_fen);
}

function fen_error() {
    console.log("FEN error");
    report_error_to_user("This error should never happen");
}

function san_error() {
    console.log("SAN error");
    report_error_to_user("Enter valid algebraic chess notation");
}

function wft_error() {
    console.log("wtf error");
    report_error_to_user("This error should never happen");
}

function report_error_to_user(my_string) {
    const errorReportId = document.getElementById("error_report");
    errorReportId.textContent = my_string;

    setTimeout(function() {errorReportId.textContent = ""},
        2500
    );
}

function switch_boards() {
    let temp = link_to_image;
    link_to_image = alt_link_to_image;
    alt_link_to_image = temp;

    display(board_fen);
}

function switch_whose_turn() {
    const whosMoveId = document.getElementById("whos_move");
    const whosMoveLabelId = document.getElementById("whos_move_label")
    if (is_white_to_move) {
        is_white_to_move = false;
        whosMoveId.textContent = "Black's";
        whosMoveLabelId.style = "color: Black";
    } else {
        is_white_to_move = true;
        whosMoveId.textContent = "White's";
        whosMoveLabelId.style = "color: White";
    }
}

init().then(() => {
    moveId.addEventListener("keypress", function(event) {
        if (event.key === "Enter") {
            event.preventDefault();
            document.getElementById("go_button").click();
        }
    });

    display(board_fen);

    window.switch_boards = switch_boards;
    window.do_move = do_move;
});