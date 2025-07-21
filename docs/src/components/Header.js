import React from 'react';
import { FaGithub } from "react-icons/fa";

const Header = () => {

  return (
    <header className="header">
      <div className="container">
        <div className="logo">
          <h1>CappuSim</h1>
          <span className="tagline">GPU-Accelerated LBM Framework</span>
        </div>
        <div className="header-links">
          <a href="https://github.com/gustavoverneck/CappuSim" target="_blank" rel="noopener noreferrer" style={{ color: "white", display: "flex", alignItems: "center", gap: "6px" }}>
            <FaGithub size={20} />
            GitHub
          </a>
        </div>
      </div>
    </header>
  );
};

export default Header;