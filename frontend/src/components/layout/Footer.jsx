import React from 'react'
import { Footer as MFooter } from 'react-materialize'
function Footer() {


    return <MFooter
        className="black"
        id="footer"
        copyrights="Powered by Linux Store Frontend v0.7.5 a8524a1"
    > <ul>
            <li>
                <a className="grey-text text-lighten-3" href="#!">Popular</a>
            </li>
            <li>
                <a className="grey-text text-lighten-3" href="#!">Editor's Choice</a>
            </li>
            <li>
                <a className="grey-text text-lighten-3" href="#!">Editor's Choice Games</a>
            </li>
            <li>
                <a className="grey-text text-lighten-3" href="#!">Browse Apps</a>
            </li>

            <li>
                <a className="grey-text text-lighten-3" href="#!">RSS Feeds</a>
            </li>
        </ul>
    </MFooter>
}

export default Footer