export interface Vector2D {
  x: number;
  y: number;
}

export class GridPosition {

  color: string;

  set position (value: Vector2D) {
    if (value.x < 0 || value.x > 7 || value.y < 0 || value.y > 7) {
      throw new Error(`GridPosition value was set out of bounds. Was ${value} but must be in 0..7`);
    }
  }


}
