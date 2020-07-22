import React, { useState } from 'react'
import 'materialize-css';
import './App.scss'

import Navbar from './components/layout/Navbar'
import Footer from './components/layout/Footer'
import Home from './components/pages/Home'
import About from './components/pages/About'
import Application from './components/pages/Application'

import {
  BrowserRouter as Router,
  Switch,
  Route,
  Link
} from "react-router-dom";
function App() {

  return (
    <Router>
      <div className="App">
        <Navbar />   
         <Switch>
          <Route path="/about">
            <About />
          </Route>
          <Route path="/apps/details/:id">
            <Application />
          </Route>
          <Route path="/">
            <Home />
          </Route>
        </Switch>
        <Footer />
      </div>
    </Router>
  )
}

export default App
