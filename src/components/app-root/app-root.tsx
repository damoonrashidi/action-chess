import { Component, Element } from '@stencil/core';
import { GridItem } from '../../lib/board';
import { Pawn } from '../../lib/pieces/pawn';

let player1 = new Player();
let player2 = new Player();


@Component({
  tag: 'app-root',
  styleUrl: 'app-root.scss',
  shadow: true,
})
export class AppRoot {

  @Element() el: HTMLElement;
  ctx: CanvasRenderingContext2D;
  WIDTH  = 800;
  HEIGHT = 800;
  SIZE   = this.WIDTH / 8;
  gridItems: GridItem[] = [];

  constructor () {
    let count = 0;
    for (let y = 0; y < 8; y++) {
      for (let x = 0; x < 8; x++) {
        this.gridItems.push(
          new GridItem(
            {x, y},
            this.SIZE,
            (++count + y) % 2 === 0 ? '#223' : '#eee',
            x == 0 ? new Pawn({x,y}, player1) : null
          )
        );
      }
    }
  }

  render() {
    return <div class="host">
      <canvas width={this.WIDTH} height={this.HEIGHT}></canvas>
    </div>
  }

  componentDidLoad() {
    this.ctx = this.el.shadowRoot.querySelector('canvas').getContext('2d');
    this.paint();
  }

  paint () {
    this.ctx.clearRect(0, 0, this.WIDTH, this.HEIGHT);
    this.drawSquares();
    //this.drawCards();
    //this.drawDeck();
  }

  drawSquares () {
    for (let i = 0; i < this.gridItems.length; i++) {
      const gridItem = this.gridItems[i];
      this.ctx.fillStyle = gridItem.color;
      this.ctx.fillRect(
        gridItem.position.x * gridItem.size,
        gridItem.position.y * gridItem.size,
        gridItem.size,
        gridItem.size,
      );
      if (gridItem.piece !== null) {
        gridItem.piece.draw(this.ctx, gridItem.position);
      }
    }
  }
}
