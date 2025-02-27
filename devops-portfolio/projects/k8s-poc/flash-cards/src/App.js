import React, { useState } from 'react';
import SystemInfo from './components/SystemInfo';
import FlashCardDeck from './components/FlashCardDeck';
import Navigation from './components/Navigation';
import './App.css';

const devopsCards = {
  kubernetes: [
    {
      front: "What is Kubernetes?",
      back: "An open-source container orchestration platform that automates deployment, scaling, and management of containerized applications.",
      category: "Kubernetes"
    },
    {
      front: "What is a Pod?",
      back: "The smallest deployable unit in Kubernetes that can contain one or more containers sharing storage and network resources.",
      category: "Kubernetes"
    }
  ],
  docker: [
    {
      front: "What is Docker?",
      back: "A platform for developing, shipping, and running applications in containers.",
      category: "Docker"
    },
    {
      front: "What is a Docker image?",
      back: "A lightweight, standalone package containing everything needed to run a piece of software.",
      category: "Docker"
    }
  ],
  cicd: [
    {
      front: "What is CI/CD?",
      back: "Continuous Integration and Continuous Delivery/Deployment - practices that enable frequent and reliable software delivery.",
      category: "CI/CD"
    },
    {
      front: "What is a pipeline?",
      back: "A series of automated steps that code goes through from development to production deployment.",
      category: "CI/CD"
    }
  ],
  git: [
    {
      front: "What is Git?",
      back: "A distributed version control system that allows multiple developers to collaborate on a project.",
      category: "Git"
    },
    {
      front: "What is a Git repository?",
      back: "A directory that contains all the files and directories of a project, along with metadata about the project.",
      category: "Git"
    }
  ],
  networking: [
    {
      front: "What is a network?",
      back: "A group of two or more devices that are connected to each other by a cable or wireless connection.",
      category: "Networking"
    },
    {
      front: "What is an IP address?",
      back: "A unique string of numbers separated by periods that identifies each computer using the Internet Protocol to communicate over a network.",
      category: "Networking"
    }
  ]
};

function App() {
  const [currentPage, setCurrentPage] = useState('home');

  const renderHomePage = () => (
    <div className="home-page">
      <h2>Welcome to the Flash Cards Application</h2>
      <p>Select a category to get started with learning!</p>
      <div className="category-grid">
        {Object.keys(devopsCards).map((category) => (
          <div 
            key={category} 
            className="category-card"
            onClick={() => {
              setCurrentPage(category);
            }}
          >
            <h3>{category.toUpperCase()}</h3>
            <p>{devopsCards[category].length} cards available</p>
          </div>
        ))}
      </div>
    </div>
  );

  const renderContent = () => {
    if (currentPage === 'system') {
      return <SystemInfo />;
    }

    if (devopsCards[currentPage]) {
      return (
        <FlashCardDeck cards={devopsCards[currentPage]} />
      );
    }

    return renderHomePage();
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>Flash Cards</h1>
        <Navigation currentPage={currentPage} setCurrentPage={setCurrentPage} />
      </header>
      <main className="App-main">
        {renderContent()}
      </main>
    </div>
  );
}

export default App; 