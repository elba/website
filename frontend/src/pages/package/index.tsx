import React from "react"
import style from "./styles.scss"
import avatar from "~/img/avatar.jpg"
import ReadmeViewer from "~/components/readme-viewer"

export const PackageDetailsPage: React.FunctionComponent = () => {
  const group = "lightyear"
  const name = "lightyear"
  const version = "0.2.1"
  const readme =
    "# Lightyear \n\n [![Build Status](https://travis-ci.org/ziman/lightyear.svg?branch=master)](https://travis-ci.org/ziman/lightyear)"
  return (
    <div className={style.page}>
      <header className={style["title"]}>
        <span className={style["title__name"]}>
          {group} / {name}
        </span>
        <span className={style["title__version"]}>{version}</span>
      </header>
      <div className={style["package-top-bar"]}>
        <a>Homepage</a>
        <a>Documentation</a>
      </div>
      <div className={style["main-layout"]}>
        <main>
          <div className={style["main-layout__readme"]}>
            <ReadmeViewer markdown={readme} />
          </div>
        </main>
        <aside className={style["main-layout__info"]}>
          <div className={style["main-layout__info__item"]}>
            <p>Install</p>
            <pre>
              "{group}/{name}" = {version}
            </pre>
          </div>
          <div className={style["main-layout__info__item"]}>
            <p>License</p>
            <a>MIT</a>
          </div>
          <div className={style["main-layout__info__item"]}>
            <p>Versions</p>
            <a>0.2.1</a>
            <a>0.2.0</a>
            <a>0.1.9</a>
            <a>0.1.8</a>
            <a>...</a>
          </div>
          <div className={style["main-layout__info__item"]}>
            <p>Owners</p>
            <div className={style["main-layout__owner"]}>
              <img
                className={style["owner__avatar"]}
                src={avatar}
                alt="avatar"
              />
              <span className={style["owner__name"]}>ziman</span>
              <span className={style["owner__email"]}>ziman@example.com</span>
            </div>
          </div>
        </aside>
      </div>
    </div>
  )
}

export default PackageDetailsPage
