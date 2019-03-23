import React from "react"
import style from "./styles.scss"
import { PackageList, PackageView } from "~/components/package_list"
import avatar from "~/img/avatar.jpg"

const testResults: PackageView[] = new Array(5).fill({
  group: "lightyear",
  name: "lightyear",
  version: "0.2.1",
  description:
    "Lightweight parser combinator library for Idris, inspired by Parsec.",
  keywords: ["parser", "parser", "parser", "parser", "parser"],
  downloads: 102,
  avatar: avatar,
  author: "ziman",
  updated_at: new Date(),
})

export const SearchResultsPage: React.FunctionComponent = () => (
  <div className={style.page}>
    <header className={style.header}>
      <b>Search result</b>
      <span className={style["search-for"]}>for</span>
      <span className={style["search-term"]}>lightyear</span>
    </header>
    <main>
      <div className={style["listing-top-bar"]}>
        <span>100 packages found</span>
        <div>
          {[1, 2, 3, 4, 5, 200].map(pageNumber => (
            <button key={pageNumber}>{pageNumber}</button>
          ))}
        </div>
      </div>
      <PackageList packages={testResults} />
    </main>
  </div>
)
