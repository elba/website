import React from "react"
import ReactDOM from "react-dom"
import { SearchResultsPage } from "./pages/search"
import { Navbar } from "./components/navbar"
import { Footer } from "./components/footer"

import "~/styles/global_styles.scss"

ReactDOM.render(
    <div>
        <Navbar />
        <SearchResultsPage />
        <Footer />
    </div>,
    document.getElementById("main-app")
)
