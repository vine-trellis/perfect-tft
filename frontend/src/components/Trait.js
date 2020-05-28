import React from "react";
import { Image } from "react-bootstrap";

import './Trait.css';

export default function Trait({ name, value }) {
  let imgPath = `traits/${name.replace(/[^a-zA-Z]+/g, '').toLowerCase()}.png`;

  return (
    <div className="Trait">
      <Image className="Trait-image" id={name} src={imgPath} />
      <div className="Trait-value">{`${value}`}</div>
    </div>
  )
}