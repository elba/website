import React from "react"
import { Link } from "react-router-dom"
import history from "~/history"
import style from "./styles.scss"

export const Navbar: React.FunctionComponent = () => (
  <nav className={style["navbar"]}>
    <Link className={style["navbar-logo"]} to="/">
      elba
    </Link>
    <div className={style["navbar-menu"]}>
      <a className={style["navbar-menu-item"]} href="#">
        Get Started
      </a>
      <a className={style["navbar-menu-item"]} href="#">
        Docs
      </a>
      <a className={style["navbar-menu-item"]} href="#">
        Log in
      </a>
    </div>
    <div className={style["search-bar"]}>
      <form
        className={style["search-form"]}
        method="GET"
        action="/search"
        onSubmit={ev => {
          ev.preventDefault()
          history.push({
            pathname: "/search",
            search: `?q=${(ev.target as any).q.value}`,
          })
        }}
      >
        <div className={style["input-container"]}>
          <input
            className={style["input-input"]}
            type="search"
            name="q"
            placeholder="search packages"
            autoComplete="off"
          />
          <span className={style["input-icon"]}>
            <i className="fas fa-search" />
          </span>
        </div>
        <button className={style["search-button"]} type="submit">
          Search
        </button>
      </form>
    </div>
  </nav>
)
