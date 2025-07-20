import React from 'react';

const Header = () => {
  return (
    <header className="header">
      <div className="container">
        <div className="logo">
          <h1>CappuSim</h1>
          <span className="tagline">GPU-Accelerated LBM Framework</span>
        </div>
        <div className="header-links">
          <a href="https://github.com/yourusername/CappuSim" target="_blank" rel="noopener noreferrer">
            GitHub
          </a>
        </div>
      </div>
    </header>
  );
};

export default Header;