import { Vector2D } from '../position';
import { Board } from '../board';
import { PieceCoalesce } from '.';
import { Player } from '../player/player';


export enum Direction {
  HORIZONTAL,
  VERTICAL,
  DIAGONAL,
  OMNI,
}

export interface Piece {
  pieceWillKill? (): void;
  pieceDidKill? (): void
  piecewillMove? (): void;
  pieceDidMove? (): void;
  pieceWillDie? (): void;
  pieceDidDie? (): void;
}

export abstract class Piece implements Piece {

  position  : Vector2D;
  maxSteps  : number = 0;
  direction : Direction;
  kills     : number = 0;
  cooldown  : number = 0;
  belongsTo : Player;

  constructor () { }

  abstract canMoveTo (destination: Vector2D, boardState: Board): boolean;
  abstract draw(ctx: CanvasRenderingContext2D, position: Vector2D): void;

  moveTo (destination: Vector2D, boardState: Board) {
    if (this.canMoveTo(destination, boardState)) {
      if (typeof this.piecewillMove === 'function') {
        this.piecewillMove();
      }
      let killPiece: PieceCoalesce = boardState.pieceAtLocation(destination);
      if (killPiece !== null) {
        this.pieceWillKill();
        this.pieceDidKill(boardState.pieceAtLocation(destination));
      }
      this.pieceDidMove();
    }
  }

  /**
   * pieceWillKill -> void
   * will be executed right before a piece consumes another piece
   */
  public pieceWillKill? (piece: PieceCoalesce): void;
  public pieceDidKill? (piece: PieceCoalesce): void;
  public pieceWillMove? (): void;
  public pieceDidMove? (): void;
  public pieceWillDie? (): void;
  public pieceDidDie? (): void;
}

