import React from "react"
import style from "./styles.scss"
import { PackageList, PackageProps } from "~/components/package-listing"
import avatar from "~/img/avatar.jpg"

const testResults: PackageProps[] = new Array(10).fill({
  group: "lightyear",
  name: "lightyear",
  version: "0.2.1",
  description:
    "Lightweight parser combinator library for Idris, inspired by Parsec.",
  keywords: ["parser", "parser", "parser", "parser", "parser"],
  downloads: 102,
  avatar: avatar,
  author: "ziman",
  updatedAt: new Date(),
})

export const Homepage: React.FunctionComponent = () => (
  <div>
    <Hero />
    <Features />
    <Stats />
    <section className={style.section}>
      <h2 className={style["section-title"]}>Popular packages</h2>
      <PackageList packages={testResults} />
    </section>
    <Question />
  </div>
)

const Hero: React.FunctionComponent = () => (
  <div className={style.hero}>
    <div className={style["hero-text"]}>
      <h1>
        Blazingly fast and modern{"\n"}
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
          className={[
            style["hero-button"],
            "button",
            "is-purple",
            "has-shadow",
          ].join(" ")}
          href="#"
        >
          Get Started
        </a>
      </div>
      <div className={style["hero-button-container"]}>
        <a
          className={[
            style["hero-button"],
            "button",
            "is-purple",
            "has-shadow",
          ].join(" ")}
          href="#"
        >
          Download
        </a>
      </div>
    </div>
  </div>
)

const Features: React.FunctionComponent = () => (
  <div className={[style.section, style.features].join(" ")}>
    <h2 className={style["section-title"]}>Features</h2>
    <ul className={style["features-item-container"]}>
      <li className={style["features-item"]}>
        <div />
        <h2>Packages</h2>
        <p>
          Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do
          eiusmod tempor incididunt ut ero labore et dolore
        </p>
      </li>
      <li className={style["features-item"]}>
        <div />
        <h2>Modern build system</h2>
        <p>
          Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do
          eiusmod tempor incididunt ut ero labore et dolore
        </p>
      </li>
      <li className={style["features-item"]}>
        <div />
        <h2>Out of box</h2>
        <p>
          Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do
          eiusmod tempor incididunt ut ero labore et dolore
        </p>
      </li>
    </ul>
  </div>
)

const Stats: React.FunctionComponent = () => (
  <div className={[style.section, style.stats].join(" ")}>
    <h2 className={style["section-title"]}>Until now, we have</h2>
    <ul className={style["stats-item-container"]}>
      <li className={style["stats-item"]}>
        <p className={style["stats-value"]}>1230</p>
        <p className={style["stats-title"]}>packages</p>
      </li>
      <li className={style["stats-item"]}>
        <p className={style["stats-value"]}>201649</p>
        <p className={style["stats-title"]}>downloads</p>
      </li>
    </ul>
  </div>
)

const Question: React.FunctionComponent = () => (
  <div className={[style.section, style.question].join(" ")}>
    <h2 className={[style["question-title"]].join(" ")}>Looks good?</h2>
    <a
      className={[
        style["question-button"],
        "button",
        "is-purple",
        "has-shadow",
      ].join(" ")}
      href="#"
    >
      Start to explore elba
    </a>
  </div>
)

export default Homepage
