import React from "react"
import { Link } from "react-router-dom"
import { GlobalStateConsumer } from "~/utils/global-state.tsx"
import history from "~/history"
import style from "./styles.scss"
import { login_by_access_token, APIROOT } from "~/api"

export const Navbar: React.FunctionComponent = () => (
  <nav className={style["navbar"]}>
    <Link className={style["navbar-logo"]} to="/">
      elba
    </Link>
    <div className={style["navbar-menu"]}>
      <a className={style["navbar-menu-item"]} href="#">
        Get Started
      </a>
      <a
        className={style["navbar-menu-item"]}
        href="https://elba.readthedocs.io/"
      >
        Docs
      </a>
      <GlobalStateConsumer>
        {globalState =>
          globalState.user === undefined ? (
            <a
              className={style["navbar-menu-item"]}
              onClick={() => {
                onLogin(globalState.fetchUser)
              }}
            >
              Log in
            </a>
          ) : (
            <Link className={style["navbar-menu-item"]} to="/profile">
              {globalState.user.name}
            </Link>
          )
        }
      </GlobalStateConsumer>
    </div>
    <div className={style["search-bar"]}>
      <form
        className={style["search-form"]}
        action="/search"
        onSubmit={onSearch}
      >
        <GlobalStateConsumer>
          {globalState => (
            <div className={style["input-container"]}>
              <input
                className={style["input-input"]}
                type="search"
                name="q"
                placeholder="search packages"
                autoComplete="off"
                onChange={e => globalState.setSearchQuery(e.target.value)}
                value={globalState.searchQuery}
              />
              <span className={style["input-icon"]}>
                <i className="fas fa-search" />
              </span>
            </div>
          )}
        </GlobalStateConsumer>
        <button className={style["search-button"]} type="submit">
          Search
        </button>
      </form>
    </div>
  </nav>
)

function onSearch(ev: React.FormEvent<HTMLFormElement>) {
  ev.preventDefault()
  history.push({
    pathname: "/search",
    search: `?q=${(ev.target as any).q.value}`,
  })
}

async function onLogin(fetchUser: () => void) {
  // let access_token = prompt("Github access token?") || ""
  // await login_by_access_token(access_token)
  window.location.assign(APIROOT + "/users/login/oauth")
}

export default Navbar
