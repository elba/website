import React, { useState, useEffect } from "react"
import style from "./styles.scss"
import { PackageList } from "~/components/package-listing"
import { PackageReq, list_groups, list_packages } from "~api"
import { RemoteData } from "~utils/remote-data"
import Navbar from "~components/navbar"
import iconBox from "~/img/box.svg"
import iconTruck from "~/img/truck.svg"
import iconRobot from "~/img/robot.svg"

export const Homepage: React.FunctionComponent = props => {
  const [packages, setPackages] = useState<RemoteData<PackageReq[]>>({
    type: "Not Ready",
  })

  useEffect(() => {
    const loadPackages = async () => {
      let groups = await list_groups()
      let packages: PackageReq[] = []
      for (var group of groups) {
        packages = packages.concat(await list_packages(group))
      }
      setPackages({ type: "Ready", data: packages })
    }
    loadPackages()
  }, [props])

  return (
    <div>
      <div className={style["strips-backgroud"]}>
        <Navbar darkTheme />
        <Hero />
      </div>
      <Features />
      <Stats />
      <section className={style.section}>
        <h2 className={style["section-title"]}>Popular packages</h2>
        <PackageList packages={packages.type === "Ready" ? packages.data : []} />
      </section>
      <Question />
    </div>
  )
}

const Hero: React.FunctionComponent = () => (
  <div className={style.hero}>
    <div className={style["hero-content"]}>
      <div className={style["hero-text"]}>
        <h1>
          A blazingly fast and modern{"\n"}
          package manager{"\n"}
          for{"\n"}
          <a
            className={style.highlight}
            href="https://www.idris-lang.org"
            target="_blank"
          >
            Idris
          </a>
        </h1>
      </div>

      <div className={style["hero-buttons"]}>
        <div className={style["hero-button-container"]}>
          <a
            className={[style["hero-button"], "button", "is-purple"].join(" ")}
            href="https://elba.readthedocs.io/en/latest/usage/quick_start.html"
            target="_blank"
          >
            Get Started
          </a>
        </div>
        <div className={style["hero-button-container"]}>
          <a
            className={[style["hero-button"], "button", "is-purple"].join(" ")}
            href="https://github.com/elba/elba/releases"
            target="_blank"
          >
            Download
          </a>
        </div>
      </div>
    </div>
  </div>
)

const Features: React.FunctionComponent = () => (
  <div className={[style.section, style.features].join(" ")}>
    <h2 className={style["section-title"]}>Features</h2>
    <ul className={style["features-item-container"]}>
      <li className={style["features-item"]}>
        <img src={iconTruck} />
        <h2>Packages</h2>
        <p>
          Many elba packages are already available online, meaning you can add
          extra dependencies to your own projects without fussing with git
          clones and ipkg installs.
        </p>
      </li>
      <li className={style["features-item"]}>
        <img src={iconRobot} />
        <h2>Modern build system</h2>
        <p>
          elba uses the state-of-the-art Pubgrub dependency resolution algorithm
          and global Nix-style caching to ensure reliable, reproducible builds.
        </p>
      </li>
      <li className={style["features-item"]}>
        <img src={iconBox} />
        <h2>Out of the box</h2>
        <p>
          Adding elba to your project is as simple as adding an `elba.toml`
          manifest file; from there, package building, doc generation, REPL
          interaction, and more all work seamlessly.
        </p>
      </li>
    </ul>
  </div>
)

const Stats: React.FunctionComponent = () => (
  <div className={[style.section, style.stats].join(" ")}>
    <h2 className={style["section-title"]}>Currently, we have</h2>
    <ul className={style["stats-item-container"]}>
      <li className={style["stats-item"]}>
        <p className={style["stats-value"]}>--</p>
        <p className={style["stats-title"]}>packages</p>
      </li>
      <li className={style["stats-item"]}>
        <p className={style["stats-value"]}>--</p>
        <p className={style["stats-title"]}>downloads</p>
      </li>
    </ul>
  </div>
)

const Question: React.FunctionComponent = () => (
  <div className={[style.section, style.question].join(" ")}>
    <h2 className={[style["question-title"]].join(" ")}>Ready to start?</h2>
    <a
      className={[style["question-button"], "button", "is-purple"].join(" ")}
      href="#"
    >
      Start exploring elba
    </a>
  </div>
)

export default Homepage
