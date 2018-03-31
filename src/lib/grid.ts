import { Vector2D } from "./position";

export class GridItem {
  constructor (
    public position: Vector2D,
    public size: number,
    public color: string,
  ) { }
}

export class GridCollection {
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
}
