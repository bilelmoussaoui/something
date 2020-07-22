import React, { useState } from 'react'
import { Button } from 'react-materialize'
function Home() {
    const handleQuickSetup = (e) => {
        e.preventDefault()
        window.location.href = "https://flatpak.org/setup/"
    }

    return <div className='container'>
        <header className="header-content">
            <h1>Apps for Linux, right here</h1>
            <p>
                Welcome to Flathub, the home of hundreds of apps which can be easily installed on any Linux distribution. Browse the apps online, from your app center or the command line.
        </p>
        </header>
        <div>
            <Button style={{ marginRight: '8px' }} onClick={handleQuickSetup}>Quick setup</Button>
            <Button>Browse the apps</Button>
        </div>

        <div className="category">
            <h2>Popular Apps</h2>
            <Button>See More</Button>
        </div>
        <div className="category">

            <h2>New & Updated Apps</h2>
            <Button>See More</Button>
        </div>

        <div className="category">
            <h2>Editor's Choice Apps</h2>
            <Button>See More</Button>
        </div>


        <div className="category">
            <h2>Editor's Choice Games</h2>
            <Button>See More</Button>
        </div>

    </div>
}

export default Home