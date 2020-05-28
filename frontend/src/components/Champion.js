import React from "react";
import { Image } from "react-bootstrap";

import './Champion.css';

export default function Champion({ champion: { name, championId, cost } }) {

  let imgPath = `champions/${name.replace(/[^a-zA-Z]+/g, '').toLowerCase()}.png`;

  return (
    <div className="Champion">
      <Image className="Champion-image" id={championId} src={imgPath} />
      <div className="Champion-level">{`${cost}`}</div>
    </div>
  )
}