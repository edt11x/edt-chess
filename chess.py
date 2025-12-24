#
import tkinter as tk


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


def _path_clear_between(pieces, c1, r1, c2, r2):
    # Check that all squares between (c1,r1) exclusive and (c2,r2) exclusive are empty.
    dc = 0 if c2 == c1 else (1 if c2 > c1 else -1)
    dr = 0 if r2 == r1 else (1 if r2 > r1 else -1)
    c = c1 + dc
    r = r1 + dr
    while (c, r) != (c2, r2):
        if _occupied_piece_at(pieces, c, r) is not None:
            return False
        c += dc
        r += dr
    return True


def is_square_attacked(pieces, col_idx, row_idx, by_color):
    # Return True if any piece of color `by_color` attacks square (col_idx,row_idx).
    for p in pieces:
        if p.color != by_color:
            continue
        pc = p.name
        c = _col_to_index(p.col)
        r = _row_to_index(p.row)
        dc = col_idx - c
        dr = row_idx - r
        # Pawn attacks
        if pc == 'Pawn':
            if p.color == 'White':
                if dr == -1 and abs(dc) == 1:
                    return True
            else:
                if dr == 1 and abs(dc) == 1:
                    return True
        elif pc == 'Knight':
            if (abs(dc), abs(dr)) in ((1, 2), (2, 1)):
                return True
        elif pc == 'Bishop':
            if abs(dc) == abs(dr) and _path_clear_between(pieces, c, r, col_idx, row_idx):
                return True
        elif pc == 'Rook':
            if (dc == 0 or dr == 0) and _path_clear_between(pieces, c, r, col_idx, row_idx):
                return True
        elif pc == 'Queen':
            if (abs(dc) == abs(dr) or dc == 0 or dr == 0) and _path_clear_between(pieces, c, r, col_idx, row_idx):
                return True
        elif pc == 'King':
            if max(abs(dc), abs(dr)) == 1:
                return True
    return False


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

    # en-passant: can capture a pawn that just moved two squares on the previous move
    global last_move
    if last_move and last_move.get('double_step'):
        lm_col, lm_row = last_move['to']
        # the double-stepped pawn must be on the same rank as the pawn
        if lm_row == row:
            if abs(lm_col - col) == 1:
                # capture by moving diagonally behind the pawn
                capture_row = row + direction
                if 0 <= capture_row < 8:
                    moves.append(f"{_index_to_col(lm_col)}{_index_to_row(capture_row)}")

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
    # castling: checks (king and rook haven't moved, path clear, and no squares are attacked)
    if piece.firstMove:
        opp = 'Black' if piece.color == 'White' else 'White'
        # white king expected at e1, black at e8
        if piece.color == 'White' and piece.col == 'e' and piece.row == 1:
            king_c = col
            king_r = row
            # king must not currently be in check
            if not is_square_attacked(pieces, king_c, king_r, opp):
                # kingside
                rook = _occupied_piece_at(pieces, _col_to_index('h'), _row_to_index(1))
                if rook and rook.name == 'Rook' and rook.firstMove:
                    f_idx = _col_to_index('f')
                    g_idx = _col_to_index('g')
                    r_idx = _row_to_index(1)
                    if _occupied_piece_at(pieces, f_idx, r_idx) is None and _occupied_piece_at(pieces, g_idx, r_idx) is None:
                        # squares the king passes through (f1,g1) must not be under attack
                        if not is_square_attacked(pieces, f_idx, r_idx, opp) and not is_square_attacked(pieces, g_idx, r_idx, opp):
                            moves.append('g1')
                # queenside
                rook = _occupied_piece_at(pieces, _col_to_index('a'), _row_to_index(1))
                if rook and rook.name == 'Rook' and rook.firstMove:
                    d_idx = _col_to_index('d')
                    c_idx = _col_to_index('c')
                    b_idx = _col_to_index('b')
                    r_idx = _row_to_index(1)
                    if _occupied_piece_at(pieces, d_idx, r_idx) is None and _occupied_piece_at(pieces, c_idx, r_idx) is None and _occupied_piece_at(pieces, b_idx, r_idx) is None:
                        # squares the king passes through (d1,c1) must not be under attack
                        if not is_square_attacked(pieces, d_idx, r_idx, opp) and not is_square_attacked(pieces, c_idx, r_idx, opp):
                            moves.append('c1')
        if piece.color == 'Black' and piece.col == 'e' and piece.row == 8:
            king_c = col
            king_r = row
            if not is_square_attacked(pieces, king_c, king_r, opp):
                # kingside
                rook = _occupied_piece_at(pieces, _col_to_index('h'), _row_to_index(8))
                if rook and rook.name == 'Rook' and rook.firstMove:
                    f_idx = _col_to_index('f')
                    g_idx = _col_to_index('g')
                    r_idx = _row_to_index(8)
                    if _occupied_piece_at(pieces, f_idx, r_idx) is None and _occupied_piece_at(pieces, g_idx, r_idx) is None:
                        if not is_square_attacked(pieces, f_idx, r_idx, opp) and not is_square_attacked(pieces, g_idx, r_idx, opp):
                            moves.append('g8')
                # queenside
                rook = _occupied_piece_at(pieces, _col_to_index('a'), _row_to_index(8))
                if rook and rook.name == 'Rook' and rook.firstMove:
                    d_idx = _col_to_index('d')
                    c_idx = _col_to_index('c')
                    b_idx = _col_to_index('b')
                    r_idx = _row_to_index(8)
                    if _occupied_piece_at(pieces, d_idx, r_idx) is None and _occupied_piece_at(pieces, c_idx, r_idx) is None and _occupied_piece_at(pieces, b_idx, r_idx) is None:
                        if not is_square_attacked(pieces, d_idx, r_idx, opp) and not is_square_attacked(pieces, c_idx, r_idx, opp):
                            moves.append('c8')

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


# --- game state and move application (for en-passant, castling, promotion) ---
last_move = None  # dict with keys: piece, from, to, double_step


def find_piece_by_pos(pieces, pos):
    # pos like 'e2'
    if len(pos) < 2:
        return None
    col = pos[0]
    row = int(pos[1])
    for p in pieces:
        if p.col == col and int(p.row) == row:
            return p
    return None


def apply_move(pieces, from_pos, to_pos, promotion=None):
    # Move a piece from from_pos (e.g. 'e2') to to_pos (e.g. 'e4').
    # Handles captures, en-passant, castling, and promotion (default to Queen).
    global last_move
    mover = find_piece_by_pos(pieces, from_pos)
    if mover is None:
        raise ValueError(f"No piece at {from_pos}")

    from_col_idx = _col_to_index(mover.col)
    from_row_idx = _row_to_index(mover.row)

    to_col = to_pos[0]
    to_row = int(to_pos[1])
    to_col_idx = _col_to_index(to_col)
    to_row_idx = _row_to_index(to_row)

    # detect en-passant capture: pawn moves diagonally to empty square
    captured = None
    if mover.name == 'Pawn':
        # en-passant target square is empty but we capture the pawn that moved two squares last move
        if _col_to_index(from_pos[0]) != to_col_idx and find_piece_by_pos(pieces, to_pos) is None:
            # diagonal move into empty square
            if last_move and last_move.get('double_step'):
                lm_to = last_move['to']
                if lm_to[0] == to_col_idx and lm_to[1] == from_row_idx:
                    # capture the pawn that made the double step
                    captured = _occupied_piece_at(pieces, lm_to[0], lm_to[1])
    # normal capture
    if captured is None:
        captured = find_piece_by_pos(pieces, to_pos)
    if captured:
        pieces.remove(captured)

    # handle castling: king moves two squares horizontally
    if mover.name == 'King' and abs(to_col_idx - from_col_idx) == 2:
        # kingside or queenside
        if to_col_idx > from_col_idx:
            # kingside: move rook from h-file to f-file
            rook_col = 'h'
            rook_target_col = 'f'
        else:
            rook_col = 'a'
            rook_target_col = 'd'
        rook = None
        for p in pieces:
            if p.name == 'Rook' and p.color == mover.color and p.col == rook_col:
                rook = p
                break
        if rook:
            rook.col = rook_target_col
            rook.firstMove = False

    # move the piece
    mover.col = to_col
    mover.row = to_row
    mover.firstMove = False

    # promotion
    if mover.name == 'Pawn':
        if (mover.color == 'White' and mover.row == 8) or (mover.color == 'Black' and mover.row == 1):
            new_name = promotion if promotion in ('Queen', 'Rook', 'Bishop', 'Knight') else 'Queen'
            mover.name = new_name

    # record last_move
    double_step = False
    if mover.name == 'Pawn' and abs(to_row_idx - from_row_idx) == 2:
        double_step = True

    last_move = {'piece': mover, 'from': (from_col_idx, from_row_idx), 'to': (to_col_idx, to_row_idx), 'double_step': double_step}



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
    # remove previously drawn pieces
    canvas.delete("piece")
    for p in pieces:
        draw_piece(canvas, p, square_size)


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

root = tk.Tk()

square_size = 50

width = height = square_size * 8

chess_canvas = tk.Canvas(root, width=width, height=height)
chess_canvas.pack()

for row in range(8):
    for col in range(8):
        x1 = col * square_size
        y1 = row * square_size
        x2 = x1 + square_size
        y2 = y1 + square_size

        if (row + col) % 2 == 0:
            chess_canvas.create_rectangle(x1, y1, x2, y2, fill='white')
        else:
            chess_canvas.create_rectangle(x1, y1, x2, y2, fill='black')

# draw all pieces from the Pieces list
draw_all_pieces(chess_canvas, Pieces, square_size)


root.mainloop()
