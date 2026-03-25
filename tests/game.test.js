const { describe, it } = require('node:test');
const assert = require('node:assert');
const TicTacToe = require('../game.js');

describe('TicTacToe', function () {
  describe('initial state', function () {
    it('should have an empty board of 9 null cells', function () {
      const game = TicTacToe.createGame();
      assert.strictEqual(game.board.length, 9);
      for (let i = 0; i < 9; i++) {
        assert.strictEqual(game.board[i], null);
      }
    });

    it('should have X as the current player', function () {
      const game = TicTacToe.createGame();
      assert.strictEqual(game.currentPlayer, 'X');
    });

    it('should have status IN_PROGRESS', function () {
      const game = TicTacToe.createGame();
      assert.strictEqual(game.getGameStatus(), 'IN_PROGRESS');
    });
  });

  describe('makeMove', function () {
    it('should place current player mark and switch turns', function () {
      const game = TicTacToe.createGame();
      const result = game.makeMove(0);
      assert.strictEqual(result, true);
      assert.strictEqual(game.board[0], 'X');
      assert.strictEqual(game.currentPlayer, 'O');
    });

    it('should reject a move on an occupied cell', function () {
      const game = TicTacToe.createGame();
      game.makeMove(0);
      const result = game.makeMove(0);
      assert.strictEqual(result, false);
      assert.strictEqual(game.board[0], 'X');
      assert.strictEqual(game.currentPlayer, 'O');
    });

    it('should reject moves after game is over', function () {
      const game = TicTacToe.createGame();
      // X wins via top row
      game.makeMove(0); // X
      game.makeMove(3); // O
      game.makeMove(1); // X
      game.makeMove(4); // O
      game.makeMove(2); // X wins

      assert.strictEqual(game.getGameStatus(), 'X_WINS');

      const result = game.makeMove(5);
      assert.strictEqual(result, false);
      assert.strictEqual(game.board[5], null);
    });
  });

  describe('checkWinner', function () {
    it('should detect X winning via top row (0, 1, 2)', function () {
      const game = TicTacToe.createGame();
      game.makeMove(0); // X
      game.makeMove(3); // O
      game.makeMove(1); // X
      game.makeMove(4); // O
      game.makeMove(2); // X wins

      assert.strictEqual(game.checkWinner(), 'X');
      assert.strictEqual(game.getGameStatus(), 'X_WINS');
    });

    it('should detect O winning via a column (1, 4, 7)', function () {
      const game = TicTacToe.createGame();
      game.makeMove(0); // X
      game.makeMove(1); // O
      game.makeMove(2); // X
      game.makeMove(4); // O
      game.makeMove(5); // X
      game.makeMove(7); // O wins

      assert.strictEqual(game.checkWinner(), 'O');
      assert.strictEqual(game.getGameStatus(), 'O_WINS');
    });

    it('should detect a win via diagonal (0, 4, 8)', function () {
      const game = TicTacToe.createGame();
      game.makeMove(0); // X
      game.makeMove(1); // O
      game.makeMove(4); // X
      game.makeMove(2); // O
      game.makeMove(8); // X wins

      assert.strictEqual(game.checkWinner(), 'X');
      assert.strictEqual(game.getGameStatus(), 'X_WINS');
    });

    it('should return null when no winner', function () {
      const game = TicTacToe.createGame();
      game.makeMove(0); // X
      assert.strictEqual(game.checkWinner(), null);
    });
  });

  describe('getWinningCells', function () {
    it('should return the winning cell indices', function () {
      const game = TicTacToe.createGame();
      game.makeMove(0); // X
      game.makeMove(3); // O
      game.makeMove(1); // X
      game.makeMove(4); // O
      game.makeMove(2); // X wins top row

      const cells = game.getWinningCells();
      assert.deepStrictEqual(cells, [0, 1, 2]);
    });

    it('should return null when no winner', function () {
      const game = TicTacToe.createGame();
      assert.strictEqual(game.getWinningCells(), null);
    });
  });

  describe('checkDraw', function () {
    it('should detect a draw when board is full with no winner', function () {
      const game = TicTacToe.createGame();
      // X O X
      // X X O
      // O X O
      game.makeMove(0); // X
      game.makeMove(1); // O
      game.makeMove(2); // X
      game.makeMove(5); // O
      game.makeMove(3); // X
      game.makeMove(6); // O
      game.makeMove(4); // X
      game.makeMove(8); // O
      game.makeMove(7); // X

      assert.strictEqual(game.checkDraw(), true);
      assert.strictEqual(game.getGameStatus(), 'DRAW');
    });

    it('should not be a draw if there is a winner', function () {
      const game = TicTacToe.createGame();
      game.makeMove(0); // X
      game.makeMove(3); // O
      game.makeMove(1); // X
      game.makeMove(4); // O
      game.makeMove(2); // X wins

      assert.strictEqual(game.checkDraw(), false);
    });
  });

  describe('resetGame', function () {
    it('should clear the board and reset to X turn', function () {
      const game = TicTacToe.createGame();
      game.makeMove(0); // X
      game.makeMove(1); // O
      game.makeMove(4); // X

      game.resetGame();

      assert.strictEqual(game.board.length, 9);
      for (let i = 0; i < 9; i++) {
        assert.strictEqual(game.board[i], null);
      }
      assert.strictEqual(game.currentPlayer, 'X');
      assert.strictEqual(game.getGameStatus(), 'IN_PROGRESS');
    });

    it('should allow new moves after reset', function () {
      const game = TicTacToe.createGame();
      // Complete a game first
      game.makeMove(0); // X
      game.makeMove(3); // O
      game.makeMove(1); // X
      game.makeMove(4); // O
      game.makeMove(2); // X wins

      assert.strictEqual(game.getGameStatus(), 'X_WINS');

      game.resetGame();

      const result = game.makeMove(4);
      assert.strictEqual(result, true);
      assert.strictEqual(game.board[4], 'X');
      assert.strictEqual(game.getGameStatus(), 'IN_PROGRESS');
    });
  });
});
