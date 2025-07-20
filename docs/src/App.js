import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Layout from './components/Layout';
import Home from './pages/Home';
import UsageGuide from './pages/UsageGuide';
import LBMTheory from './pages/LBMTheory';
import Documentation from './pages/Documentation';
import './App.css';

function App() {
  return (
    <Router basename="/CappuSim">
      <Layout>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/usage-guide" element={<UsageGuide />} />
          <Route path="/lbm-theory" element={<LBMTheory />} />
          <Route path="/documentation" element={<Documentation />} />
        </Routes>
      </Layout>
    </Router>
  );
}

export default App;