import { Piece, Direction } from "./piece";
import { Vector2D } from '../position';

export class Pawn extends Piece {

  kills    : number = 0;
  cooldown : number =  500;

  constructor (
    public position: Vector2D,
    public maxSteps: number,
    public direction: Direction,
  ) {
    super();
  }

  canMoveTo (destination: Vector2D): boolean {
    if (this.position.y == destination.y - 1) return true;
  }


}
