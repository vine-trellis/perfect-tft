import React from 'react';
import Champion from './Champion';
import Trait from './Trait';

import { Row, Col } from 'react-bootstrap';

import './Team.css';

export default function Team({ team: { champions, traits } }) {

  return (
    <div className="Team">
      <Row>
        <Col>
          <Row>
            {
              champions.map(champion =>
                <Col xs={2}>
                  <Champion key={champion.championId} champion={champion} />
                </Col>
              )
            }
          </Row>
        </Col>
        <Col>
          <Row>
            {
              Object.entries(traits).map(([name, value]) =>
                <Col xs={2}>
                  <Trait key={name} name={name} value={value} />
                </Col>
              )
            }
          </Row>
        </Col>
      </Row>
    </div>
  )
}