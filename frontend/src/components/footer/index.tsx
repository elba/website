import React from "react"
import style from "./styles.scss"

export const Footer: React.FunctionComponent = () => (
  <footer className={style["footer"]}>
    <ul className={style["items"]}>
      <li className={style["item"]}>
        <h2>Help</h2>
        <a href="#">Get started</a>
        <a href="#">Docs</a>
        <a href="#">Guide</a>
      </li>
      <li className={style["item"]}>
        <h2>Get in touch</h2>
        <a href="https://github.com/elba/elba/issues" target="_blank">
          Issue Tracker (GitHub)
        </a>
        <a href="#">Riot</a>
      </li>
      <li className={style["item"]}>
        <h2>Link</h2>
        <a href="https://www.idris-lang.org" target="_blank">
          Idris
        </a>
        <a href="https://github.com/elba/elba" target="_blank">
          GitHub
        </a>
      </li>
    </ul>
  </footer>
)

export default Footer
