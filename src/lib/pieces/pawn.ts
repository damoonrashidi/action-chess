import { Piece, Direction } from "./piece";
import { Vector2D } from '../position';
import { Board } from "../board";
import { Player } from "../player/player";

export class Pawn extends Piece {

  kills     = 0;
  cooldown  = 500;
  maxSteps  = 1;
  direction = Direction.VERTICAL;

  constructor (
    public position: Vector2D,
    private player: Player,
  ) {
    super();
  }

  draw (ctx: CanvasRenderingContext2D, position: Vector2D): void {
    ctx.arc(position.x * 50, position.y * 50, 10, 0, Math.PI * 2);
    ctx.fillStyle = this.player.color;
    ctx.fill();
  }

  canMoveTo (destination: Vector2D, boardState: Board): boolean {
    boardState;
    if (this.position.y == destination.y - 1) return true;
  }


}
