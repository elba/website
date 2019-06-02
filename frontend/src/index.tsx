import React from "react"
import ReactDOM from "react-dom"
import { Router, Route } from "react-router-dom"
import history from "~/history"
import { UserProvider } from "~/utils/user-context.tsx"
import Navbar from "~/components/navbar"
import Footer from "~/components/footer"
import Homepage from "~/pages/home"
import SearchResultsPage from "~/pages/search"
import PackageDetailsPage from "~/pages/package"
import UserProfilePage from "~/pages/profile"

import "~/styles/global_styles.scss"

ReactDOM.render(
  <UserProvider>
    <Router history={history}>
      <Navbar />
      <Route exact path="/" component={Homepage} />
      <Route exact path="/search" component={SearchResultsPage} />
      <Route path="/package" component={PackageDetailsPage} />
      <Route path="/profile" component={UserProfilePage} />
      <Footer />
    </Router>
  </UserProvider>,
  document.getElementById("main-app")
)
