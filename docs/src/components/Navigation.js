import React from 'react';
import { Link, useLocation } from 'react-router-dom';

const Navigation = () => {
  const location = useLocation();

  return (
    <nav className="navigation">
      <div className="container">
        <ul className="nav-links">
          <li>
            <Link to="/" className={location.pathname === '/' ? 'active' : ''}>
              Home
            </Link>
          </li>
          <li>
            <Link to="/usage-guide" className={location.pathname === '/usage-guide' ? 'active' : ''}>
              Usage Guide
            </Link>
          </li>
          <li>
            <Link to="/lbm-theory" className={location.pathname === '/lbm-theory' ? 'active' : ''}>
              LBM Theory
            </Link>
          </li>
          <li>
            <Link to="/documentation" className={location.pathname === '/documentation' ? 'active' : ''}>
              Documentation
            </Link>
          </li>
        </ul>
      </div>
    </nav>
  );
};

export default Navigation;