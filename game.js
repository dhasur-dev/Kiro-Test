const TicTacToe = {
  createGame: function () {
    const WIN_CONDITIONS = [
      [0, 1, 2],
      [3, 4, 5],
      [6, 7, 8],
      [0, 3, 6],
      [1, 4, 7],
      [2, 5, 8],
      [0, 4, 8],
      [2, 4, 6],
    ];

    const game = {
      board: Array(9).fill(null),
      currentPlayer: 'X',

      makeMove: function (index) {
        if (index < 0 || index > 8) {
          return false;
        }
        if (game.board[index] !== null) {
          return false;
        }
        if (game.getGameStatus() !== 'IN_PROGRESS') {
          return false;
        }
        game.board[index] = game.currentPlayer;
        game.currentPlayer = game.currentPlayer === 'X' ? 'O' : 'X';
        return true;
      },

      checkWinner: function () {
        for (var i = 0; i < WIN_CONDITIONS.length; i++) {
          var a = WIN_CONDITIONS[i][0];
          var b = WIN_CONDITIONS[i][1];
          var c = WIN_CONDITIONS[i][2];
          if (
            game.board[a] !== null &&
            game.board[a] === game.board[b] &&
            game.board[a] === game.board[c]
          ) {
            return game.board[a];
          }
        }
        return null;
      },

      getWinningCells: function () {
        for (var i = 0; i < WIN_CONDITIONS.length; i++) {
          var a = WIN_CONDITIONS[i][0];
          var b = WIN_CONDITIONS[i][1];
          var c = WIN_CONDITIONS[i][2];
          if (
            game.board[a] !== null &&
            game.board[a] === game.board[b] &&
            game.board[a] === game.board[c]
          ) {
            return [a, b, c];
          }
        }
        return null;
      },

      checkDraw: function () {
        if (game.checkWinner() !== null) {
          return false;
        }
        for (var i = 0; i < game.board.length; i++) {
          if (game.board[i] === null) {
            return false;
          }
        }
        return true;
      },

      getGameStatus: function () {
        var winner = game.checkWinner();
        if (winner === 'X') {
          return 'X_WINS';
        }
        if (winner === 'O') {
          return 'O_WINS';
        }
        if (game.checkDraw()) {
          return 'DRAW';
        }
        return 'IN_PROGRESS';
      },

      resetGame: function () {
        game.board = Array(9).fill(null);
        game.currentPlayer = 'X';
      },
    };

    return game;
  },
};

if (typeof module !== 'undefined' && module.exports) {
  module.exports = TicTacToe;
}
