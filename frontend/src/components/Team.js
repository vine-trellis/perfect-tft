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
                <Col lg={2} md={3} sm={4} xs={6} key={champion.championId}>
                  <Champion champion={champion} />
                </Col>
              )
            }
          </Row>
        </Col>
        <Col>
          <Row>
            {
              Object.entries(traits).map(([name, value]) =>
                <Col lg={2} md={3} sm={4} xs={6} key={name}>
                  <Trait name={name} value={value} />
                </Col>
              )
            }
          </Row>
        </Col>
      </Row>
    </div>
  )
}