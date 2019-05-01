import React from "react"
import style from "./styles.scss"
import avatar from "~/img/avatar.jpg"

export const PackageDetailsPage: React.FunctionComponent = () => {
  const group = "lightyear"
  const name = "lightyear"
  const version = "0.2.1"
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
            {/* readme */}
            <h1>Insert markdown here</h1>
            <p>
              Lorem ipsum dolor sit amet, consectetur adipisicing elit. Sit
              blanditiis harum qui hic rerum quisquam iure, placeat non
              perspiciatis beatae voluptatum vero corrupti sed velit atque!
              Vitae earum facilis sunt.
            </p>
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
            <div className={style["owner-layout"]}>
              <img
                className={style["owner-layout__avatar"]}
                src={avatar}
                alt="avatar"
              />
              <span className={style["owner-layout__name"]}>
                ziman ziman@example.com
              </span>
            </div>
          </div>
        </aside>
      </div>
    </div>
  )
}

export default PackageDetailsPage
