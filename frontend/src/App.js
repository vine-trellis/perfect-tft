import React from 'react';
import './App.css';
import Team from './components/Team';

import { Container, Card, ListGroup, ListGroupItem } from 'react-bootstrap';
import Layout from './components/Layout';

import level1 from './data/level1.json';
import level2 from './data/level2.json';
import level3 from './data/level3.json';
import level4 from './data/level4.json';
import level5 from './data/level5.json';
import level6 from './data/level6.json';
import level7 from './data/level7.json';
import level8 from './data/level8.json';
import level9 from './data/level9.json';

import 'bootstrap/dist/css/bootstrap.min.css';

function App() {
  let perfectCompJsons = [
    level1,
    level2,
    level3,
    level4,
    level5,
    level6,
    level7,
    level8,
    level9,
  ];

  return (
    <Layout>
      <Container className="App-container">
        {
          perfectCompJsons.map((json, idx) =>
            <Card className="Team-card" key={idx}>
              <Card.Header>{`Level ${idx + 1} perfect comps`}</Card.Header>
              <ListGroup>
                {
                  json.map((team, idx) => <ListGroupItem key={idx}><Team team={team} /></ListGroupItem>)
                }
              </ListGroup>

            </Card>
          )
        }
      </Container>
    </Layout>
  );
}

export default App;
