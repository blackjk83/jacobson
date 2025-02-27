import React from 'react';
import './Navigation.css';

const Navigation = ({ currentPage, setCurrentPage }) => {
  return (
    <nav className="main-nav">
      <button 
        className={currentPage === 'cards' ? 'active' : ''} 
        onClick={() => setCurrentPage('cards')}
      >
        Flash Cards
      </button>
      <button 
        className={currentPage === 'system' ? 'active' : ''} 
        onClick={() => setCurrentPage('system')}
      >
        System Info
      </button>
    </nav>
  );
};

export default Navigation; 