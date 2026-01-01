import sys
import chess
from game import ChessGame
from ai import ChessAI


def ask_player_color():
    """Ask the user which color they want to play as."""
    prompt = 'Play as White or Black? [W/b]: '
    while True:
        try:
            choice = input(prompt).strip().lower()
        except (EOFError, KeyboardInterrupt):
            print('\nDefaulting to White')
            return 'White'
        if choice == '' or choice in ('w', 'white'):
            return 'White'
        if choice in ('b', 'black'):
            return 'Black'
        print('Please enter W (White) or B (Black).')


def repl(game, ai, player_color):
    """Main REPL loop for the chess game."""
    computer_color = chess.BLACK if player_color == 'White' else chess.WHITE
    print(f'Chess Practice Application. You are playing {player_color}. Computer plays as {"Black" if player_color == "White" else "White"}.')
    print('Type "help" for commands.')

    while True:
        if game.board.turn == computer_color:
            print("Computer's turn...")
            move = ai.get_best_move(game.board)
            if move:
                game.make_move(move)
                print(f'Computer plays {move}')
                print(game.render_board())
            else:
                print('No moves available.')
                break
            if game.is_game_over():
                winner = game.get_winner()
                if winner:
                    print(f'Checkmate! {winner} wins.')
                else:
                    print('Game ended in draw.')
                break
            continue

        try:
            cmd = input('> ').strip()
        except (EOFError, KeyboardInterrupt):
            print('\nExiting.')
            break

        if not cmd:
            continue

        if cmd in ('q', 'quit', 'exit'):
            break

        if cmd in ('h', 'help'):
            print('Commands:')
            print('  board | show        - display board')
            print('  moves <square>      - show legal moves for piece at square (e.g. moves e2)')
            print('  move <from> <to>    - make a move (e.g. move e2 e4)')
            print('  hint                - get a hint for the best move')
            print('  undo                - undo the last move')
            print('  eval                - show current position evaluation')
            print('  quit | q            - exit')
            continue

        if cmd in ('board', 'show'):
            print(game.render_board())
            continue

        if cmd.startswith('moves '):
            parts = cmd.split()
            if len(parts) < 2:
                print('usage: moves <square>')
                continue
            sq = parts[1]
            moves = game.get_legal_moves_for_square(sq)
            if moves:
                piece_info = game.get_piece_info(sq)
                if piece_info:
                    print(f'Legal moves for {piece_info["color"]} {piece_info["name"]} at {sq}: {moves}')
                else:
                    print(f'Legal moves from {sq}: {moves}')
            else:
                print('No piece at', sq, 'or no legal moves.')
            continue

        if cmd.startswith('move '):
            parts = cmd.split()
            if len(parts) < 3:
                print('usage: move <from> <to>')
                continue
            from_sq, to_sq = parts[1], parts[2]
            move_str = from_sq + to_sq
            if game.make_move(move_str):
                print(f'You played {move_str}')
                print(game.render_board())
                if game.is_game_over():
                    winner = game.get_winner()
                    if winner:
                        print(f'Checkmate! {winner} wins.')
                    else:
                        print('Game ended in draw.')
                    break
            else:
                print('Illegal move:', move_str)
            continue

        if cmd == 'hint':
            hint = ai.get_hint(game.board)
            if hint:
                print(f'Suggested move: {hint["move"]} (evaluation: {hint["evaluation"]})')
            else:
                print('No hint available.')
            continue

        if cmd == 'undo':
            if game.undo_move():
                print('Undid last move.')
                print(game.render_board())
            else:
                print('No moves to undo.')
            continue

        if cmd == 'eval':
            eval_score = ai.evaluate_board(game.board)
            print(f'Current position evaluation: {eval_score}')
            continue

        print('Unknown command. Type "help" for commands.')


def run_practice_app():
    """Main entry point for the practice application."""
    print('Welcome to Chess Practice Application!')
    player_color = ask_player_color()

    game = ChessGame()
    ai = ChessAI(depth=3)  # Default medium difficulty

    print(game.render_board())
    repl(game, ai, player_color)