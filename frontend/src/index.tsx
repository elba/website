import React from "react"
import ReactDOM from "react-dom"
import { Router, Route } from "react-router-dom"
import history from "~/history"
import { GlobalStateProvider } from "~/utils/global-state.tsx"
import Navbar from "~/components/navbar"
import Footer from "~/components/footer"
import Homepage from "~/pages/home"
import SearchResultsPage from "~/pages/search"
import PackageDetailsPage from "~/pages/package"
import UserProfilePage from "~/pages/profile"

import "~/styles/global_styles.scss"

ReactDOM.render(
  <GlobalStateProvider>
    <Router history={history}>
      <Navbar />
      <Route exact path="/" component={Homepage} />
      <Route exact path="/search" component={SearchResultsPage} />
      <Route
        path="/package/:group/:package/:version"
        component={PackageDetailsPage}
      />
      <Route path="/profile" component={UserProfilePage} />
      <Footer />
    </Router>
  </GlobalStateProvider>,
  document.getElementById("main-app")
)
