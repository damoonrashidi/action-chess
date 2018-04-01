import { Vector2D } from "./position";
import { PieceCoalesce } from "./pieces";

export class GridItem {
  constructor (
    public position: Vector2D,
    public size: number,
    public color: string,
    public piece: PieceCoalesce | null,
  ) { }
}

export class Board {
  private grid: GridItem[];

  constructor (private size: number) {}

  addItem (item: GridItem) {
    if  (
      item.position.x < 0
      || item.position.x >= this.size
      || item.position.y < 0
      || item.position.y >= this.size
    ) {
      console.warn(`Tried to add grid item [${item.position.x, item.position.y}] out of range 0..${this.size}`)
    }
    this.grid.push(item);
  }

  pieceAtLocation (position: Vector2D): PieceCoalesce | null {
    return this.grid.find(location =>
      location.position.x === position.x
      && location.position.y === position.y
    ).piece;
  }

  getIndexOfLocation (position: Vector2D): number {
    for (let i = 0; i < this.grid.length; i++) {
      if (this.grid[i].position.x === position.x && this.grid[i].position.y) {
        return i;
      }
    }
  }

  kill (position: Vector2D) {
    this.grid[this.getIndexOfLocation(position)].piece = null;
  }
}
