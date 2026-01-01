#
import sys
import io
import unittest


class Piece:
    def __init__(self, color, name, col, row):
        self.color = color
        self.name = name
        self.col = col
        self.row = row
        self.firstMove = True

    def printNewPosition(self, col, row):
        print(self.color, self.name, 'at :', col, row)

    def printPosition(self):
        print(self.color, self.name, 'at :', self.col, self.row)


def initPositions(array, color, name, columns, row):
    for c in columns:
        array.append(Piece(color, name, c, row))


def isSquareOccupied(array, col, row):
    ret = False
    for x in range(len(array)):
        if ((array[x].col == col) and (array[x].row == row)):
            ret = True
    return ret


def randomMovePiece(array, color):
    for x in range(len(Pieces)):
        if (Pieces[x].color == color):
            print('Try to move', Pieces[x].color, Pieces[x].name, 'at :', Pieces[x].col, Pieces[x].row)


def possiblePawnMoves(array, color, piece):
    if (piece.name == 'Pawn') and (piece.row != 8) and (piece.row != 1) and (piece.color == color):
        # walk through the array and see if we can move one space forward?
        if (piece.color == 'White') and (isSquareOccupied(array, piece.col, piece.row + 1) == False):
            print(' possible move to', end=' ')
            piece.printNewPosition(piece.col, piece.row + 1)
        if (piece.color == 'Black') and (isSquareOccupied(array, piece.col, piece.row - 1) == False):
            print(' possible move to', end=' ')
            piece.printNewPosition(piece.col, piece.row - 1)
        # walk through the array and see if we can move two spaces forward?
        if (piece.color == 'White') and (isSquareOccupied(array, piece.col, piece.row + 1) == False):
            if (piece.row == 2) and (piece.firstMove) and (isSquareOccupied(array, piece.col, piece.row + 2) == False):
                print(' possible move to', end=' ')
                piece.printNewPosition(piece.col, piece.row + 2)
        if (piece.color == 'Black') and (isSquareOccupied(array, piece.col, piece.row - 1) == False):
            if (piece.row == 7) and (piece.firstMove) and (isSquareOccupied(array, piece.col, piece.row - 2) == False):
                print(' possible move to', end=' ')
                piece.printNewPosition(piece.col, piece.row - 2)
    # walk through the array and see if we can take left?
    # walk through the array and see if we can take right?
    # walk through the array and check to see if we can take en passant?
    # check to see if we are promoted.


def possibleMoves(array, color, piece):
    if (piece.color == color):
        if (piece.name == 'Pawn'):
            if (piece.firstMove == True):
                print('Pawn can move one or two spaces')


def _col_to_index(col):
    # Accept either a character 'a'..'h' or a 0-based integer
    if isinstance(col, int):
        return col
    try:
        return ord(col.lower()) - ord('a')
    except Exception:
        return 0


def _row_to_index(row):
    # Convert chess row (1..8) to 0-based canvas row (0 at top)
    try:
        return 8 - int(row)
    except Exception:
        return int(row)


def _index_to_col(i):
    return chr(ord('a') + i)


def _index_to_row(r):
    # convert 0-based canvas row to chess row number
    return 8 - r


def _occupied_piece_at(pieces, col_idx, row_idx):
    for p in pieces:
        if _col_to_index(p.col) == col_idx and _row_to_index(p.row) == row_idx:
            return p
    return None


def get_pawn_moves(pieces, piece):
    moves = []
    col = _col_to_index(piece.col)
    row = _row_to_index(piece.row)
    # white moves "up" (decreasing row index), black moves "down"
    direction = -1 if piece.color == 'White' else 1

    # one square forward
    nrow = row + direction
    if 0 <= nrow < 8 and _occupied_piece_at(pieces, col, nrow) is None:
        moves.append(f"{_index_to_col(col)}{_index_to_row(nrow)}")
        # two squares from starting rank
        start_row = 6 if piece.color == 'White' else 1
        nrow2 = row + 2 * direction
        if row == start_row and 0 <= nrow2 < 8 and _occupied_piece_at(pieces, col, nrow2) is None:
            moves.append(f"{_index_to_col(col)}{_index_to_row(nrow2)}")

    # captures
    for dcol in (-1, 1):
        c = col + dcol
        r = row + direction
        if 0 <= c < 8 and 0 <= r < 8:
            target = _occupied_piece_at(pieces, c, r)
            if target is not None and target.color != piece.color:
                moves.append(f"{_index_to_col(c)}{_index_to_row(r)}")

    return moves


def _sliding_moves(pieces, piece, directions):
    moves = []
    col = _col_to_index(piece.col)
    row = _row_to_index(piece.row)
    for dcol, drow in directions:
        c, r = col + dcol, row + drow
        while 0 <= c < 8 and 0 <= r < 8:
            target = _occupied_piece_at(pieces, c, r)
            if target is None:
                moves.append(f"{_index_to_col(c)}{_index_to_row(r)}")
            else:
                if target.color != piece.color:
                    moves.append(f"{_index_to_col(c)}{_index_to_row(r)}")
                break
            c += dcol
            r += drow
    return moves


def get_rook_moves(pieces, piece):
    directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]
    return _sliding_moves(pieces, piece, directions)


def get_bishop_moves(pieces, piece):
    directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)]
    return _sliding_moves(pieces, piece, directions)


def get_queen_moves(pieces, piece):
    return get_rook_moves(pieces, piece) + get_bishop_moves(pieces, piece)


def get_knight_moves(pieces, piece):
    moves = []
    col = _col_to_index(piece.col)
    row = _row_to_index(piece.row)
    deltas = [(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2)]
    for dcol, drow in deltas:
        c = col + dcol
        r = row + drow
        if 0 <= c < 8 and 0 <= r < 8:
            target = _occupied_piece_at(pieces, c, r)
            if target is None or target.color != piece.color:
                moves.append(f"{_index_to_col(c)}{_index_to_row(r)}")
    return moves


def get_king_moves(pieces, piece):
    moves = []
    col = _col_to_index(piece.col)
    row = _row_to_index(piece.row)
    for dcol in (-1, 0, 1):
        for drow in (-1, 0, 1):
            if dcol == 0 and drow == 0:
                continue
            c = col + dcol
            r = row + drow
            if 0 <= c < 8 and 0 <= r < 8:
                target = _occupied_piece_at(pieces, c, r)
                if target is None or target.color != piece.color:
                    moves.append(f"{_index_to_col(c)}{_index_to_row(r)}")
    return moves


def get_piece_legal_moves(pieces, piece):
    if piece.name == 'Pawn':
        return get_pawn_moves(pieces, piece)
    if piece.name == 'Rook':
        return get_rook_moves(pieces, piece)
    if piece.name == 'Knight':
        return get_knight_moves(pieces, piece)
    if piece.name == 'Bishop':
        return get_bishop_moves(pieces, piece)
    if piece.name == 'Queen':
        return get_queen_moves(pieces, piece)
    if piece.name == 'King':
        return get_king_moves(pieces, piece)
    return []


def piece_unicode(name, color):
    # Map piece name and color to a unicode chess glyph
    glyphs = {
        'Pawn':   {'White': '\u2659', 'Black': '\u265F'},
        'Rook':   {'White': '\u2656', 'Black': '\u265C'},
        'Knight': {'White': '\u2658', 'Black': '\u265E'},
        'Bishop': {'White': '\u2657', 'Black': '\u265D'},
        'Queen':  {'White': '\u2655', 'Black': '\u265B'},
        'King':   {'White': '\u2654', 'Black': '\u265A'},
    }
    return glyphs.get(name, {}).get(color, '?')


def draw_piece(canvas, piece, square_size):
    col_idx = _col_to_index(piece.col)
    row_idx = _row_to_index(piece.row)

    # center of the square
    cx = col_idx * square_size + square_size / 2
    cy = row_idx * square_size + square_size / 2

    glyph = piece_unicode(piece.name, piece.color)
    size = int(square_size * 0.6)
    # choose a fill color that contrasts with the square color
    square_is_dark = ((row_idx + col_idx) % 2) == 1
    fill_color = "white" if square_is_dark else "black"
    # use a tag so pieces can be cleared/redrawn later
    canvas.create_text(cx, cy, text=glyph, font=("DejaVu Sans", size), tags=("piece",), fill=fill_color)


def draw_all_pieces(canvas, pieces, square_size):
    # legacy tkinter helper (kept for reference) - no-op in terminal mode
    return


# initialize the board
Pieces = []
initPositions(Pieces, 'White', 'Pawn', 'abcdefgh', 2)
initPositions(Pieces, 'White', 'Rook', 'ah', 1)
initPositions(Pieces, 'White', 'Knight', 'bg', 1)
initPositions(Pieces, 'White', 'Bishop', 'cf', 1)
initPositions(Pieces, 'White', 'Queen', 'd', 1)
initPositions(Pieces, 'White', 'King', 'e', 1)

initPositions(Pieces, 'Black', 'Pawn', 'abcdefgh', 7)
initPositions(Pieces, 'Black', 'Rook', 'ah', 8)
initPositions(Pieces, 'Black', 'Knight', 'bg', 8)
initPositions(Pieces, 'Black', 'Bishop', 'cf', 8)
initPositions(Pieces, 'Black', 'Queen', 'd', 8)
initPositions(Pieces, 'Black', 'King', 'e', 8)

# Print positions of white pawns, to test the initialization
for x in range(len(Pieces)):
    if ((Pieces[x].name == 'Pawn') and (Pieces[x].color == 'White')):
        Pieces[x].printPosition()
        moves = get_piece_legal_moves(Pieces, Pieces[x])
        print(' legal moves:', moves)
print(' ')
# Print positions of black pawns, to test the initialization
for x in range(len(Pieces)):
    if ((Pieces[x].name == 'Pawn') and (Pieces[x].color == 'Black')):
        Pieces[x].printPosition()
        moves = get_piece_legal_moves(Pieces, Pieces[x])
        print(' legal moves:', moves)
print(' ')


def render_board_terminal(pieces):
    board = [['.' for _ in range(8)] for _ in range(8)]
    for p in pieces:
        c = _col_to_index(p.col)
        r = _row_to_index(p.row)
        board[r][c] = piece_unicode(p.name, p.color)

    files = 'a b c d e f g h'
    print('  ' + files)
    for r in range(8):
        rank = 8 - r
        row_str = str(rank) + ' '
        for c in range(8):
            row_str += board[r][c] + ' '
        print(row_str + str(rank))
    print('  ' + files)


def run_unit_tests():
    """Run unit tests under the `tests` directory and return the textual output."""
    loader = unittest.TestLoader()
    suite = loader.discover(start_dir='tests', pattern='test_*.py')
    buf = io.StringIO()
    runner = unittest.TextTestRunner(stream=buf, verbosity=2)
    result = runner.run(suite)
    return buf.getvalue(), result


def find_piece_by_square(pieces, square):
    if not square or len(square) < 2:
        return None
    col = square[0]
    row = square[1:]
    for p in pieces:
        if p.col == col and str(p.row) == row:
            return p
    return None


def list_pieces(pieces):
    for p in pieces:
        print(p.color, p.name, 'at', p.col + str(p.row))


def ask_player_color():
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


def repl(pieces, player_color):
    print(f'Terminal chess viewer. You are playing {player_color}. Type "help" for commands.')
    while True:
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
            print('  list                - list all pieces')
            print('  moves <square>      - show legal moves for piece at square (e.g. moves e2)')
            print('  quit | q            - exit')
            continue
        if cmd in ('board', 'show'):
            render_board_terminal(pieces)
            continue
        if cmd == 'list':
            list_pieces(pieces)
            continue
        if cmd.startswith('moves '):
            parts = cmd.split()
            if len(parts) < 2:
                print('usage: moves <square>')
                continue
            sq = parts[1]
            p = find_piece_by_square(pieces, sq)
            if p is None:
                print('No piece at', sq)
                continue
            moves = get_piece_legal_moves(pieces, p)
            print('Legal moves for', p.color, p.name, p.col + str(p.row), ':', moves)
            continue
        print('Unknown command. Type "help" for commands.')


if __name__ == '__main__':
    # If invoked with --run-tests, run unit tests and display results above the board
    if '--run-tests' in sys.argv or '--test' in sys.argv:
        output, result = run_unit_tests()
        print('\n=== UNIT TEST OUTPUT ===')
        print(output)
        print('=== SUMMARY ===')
        print(f"Ran: {result.testsRun}, Failures: {len(result.failures)}, Errors: {len(result.errors)}")
        print('\nShowing board after tests:\n')
        render_board_terminal(Pieces)
        # continue into REPL after showing results

    player_color = ask_player_color()
    render_board_terminal(Pieces)
    repl(Pieces, player_color)
