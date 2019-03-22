import React from "react"
import ReactDOM from "react-dom"
import { BrowserRouter as Router, Route, Redirect } from "react-router-dom";
import { Navbar } from "./components/navbar"
import { Footer } from "./components/footer"
import { Homepage } from "./pages/home"
import { SearchResultsPage } from "./pages/search"

import "~/styles/global_styles.scss"

ReactDOM.render(
    <div>
        <Navbar />
        <Router>
            <Route exact path="/" component={Homepage} />
            <Route exact path="/search" component={SearchResultsPage} />
            {/* <Redirect to="/" /> */}
        </Router>
        <Footer />
    </div>,
    document.getElementById("main-app")
)
