import React from "react"
import style from "./styles.scss"

export const PackageDetailsPage: React.FunctionComponent = () => {
  const group = "lightyear"
  const name = "lightyear"
  const version = "0.2.1"
  return (
    <div className={style.page}>
      <header>
        <span>
          {group}/{name}
        </span>
        <span>{version}</span>
      </header>
      <div className={style["main-layout"]}>
        <main>
          <div>
            <a>Homepage</a>
            <a>Documentation</a>
          </div>
          <div>
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
        <aside>
          <div>
            <p>Install</p>
            <code>
              > elba add {group}/{name}
            </code>
          </div>
          <div>
            <p>License</p>
            <a>MIT</a>
          </div>
          <div>
            <p>Versions</p>
            <a>0.2.1</a>
            <a>0.2.0</a>
            <a>0.1.9</a>
            <a>0.1.8</a>
            <a>Older...</a>
          </div>
        </aside>
      </div>
    </div>
  )
}

export default PackageDetailsPage
