import React from "react"
import style from "./styles.scss"

export const Navbar: React.FunctionComponent = () => (
    <nav className={style["navbar"]}>
        <a className={style["navbar-logo"]} href="#">
            elba
        </a>
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
            <form className={style["search-form"]} method="GET" action="/search">
                <div className={style["input-container"]}>
                    <input className={style["input-input"]} type="search" name="q" placeholder="search packages"></input>
                    <span className={style["input-icon"]}>
                        <i className="fas fa-search"></i>
                    </span>
                </div>
                <button className={style["search-button"]} type="submit">Search</button>
            </form>
        </div>
    </nav>
);

