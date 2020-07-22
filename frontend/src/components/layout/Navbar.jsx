import React, { useState } from 'react'
import { Navbar as MNavbar, Icon, NavItem } from 'react-materialize'
import FlathubLogo from './../../../assets/flathub-logo-toolbar.svg'
function Navbar() {

    return <MNavbar
        alignLinks="right"
        brand={<a className="brand-logo" href="#"><img src={FlathubLogo} alt="Flathub" /></a>}
        id="mobile-nav"
        className="navbar"
        menuIcon={<Icon>menu</Icon>}
        options={{
            draggable: true,
            edge: 'left',
            inDuration: 250,
            onCloseEnd: null,
            onCloseStart: null,
            onOpenEnd: null,
            onOpenStart: null,
            outDuration: 200,
            preventScrolling: true
        }}
    >
        <NavItem href="">
            Applications
        </NavItem>
        <NavItem href="https://github.com/flathub/flathub/wiki/App-Submission">
            Publish
        </NavItem>
        <NavItem href="https://discourse.flathub.org/">
            Forum
        </NavItem>
        <NavItem href="components.html">
            About
        </NavItem>
    </MNavbar>

}

export default Navbar