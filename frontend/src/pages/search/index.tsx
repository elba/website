import React, { useState, useEffect } from "react"
import style from "./styles.scss"
import { PackageList, PackageProps } from "~components/package-listing"
import { RemoteData } from "~/utils/remote-data"
import avatar from "~/img/avatar.jpg"

const testResults: PackageProps[] = new Array(5)
  .fill({
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
  .map((item, idx) => ({ ...item, name: `${item.name}${idx}` }))

export const SearchResultsPage: React.FunctionComponent = () => {
  const [result, setResult] = useState<RemoteData<PackageProps[]>>({
    type: "Not Asked",
  })
  useEffect(() => {
    if (result.type === "Not Asked") {
      setResult({ type: "Started" })
      setTimeout(() => {
        setResult({ type: "Done", data: testResults })
      }, 1000)
    }
  })
  return (
    <div className={style.page}>
      <header className={style.header}>
        <b>Search results</b>
        <span className={style["search-for"]}>for</span>
        <span className={style["search-term"]}>lightyear</span>
      </header>
      {result.type === "Done" ? (
        <main>
          <div className={style["listing-top-bar"]}>
            <span className={style["packages-found-label"]}>
              100 packages found
            </span>
            <div
              className={style["listing-top-bar__pagination-button-container"]}
            >
              {[1, 2, 3, 4, 5, 200].map(pageNumber => (
                <a className={style["pagination-button"]} key={pageNumber}>
                  {pageNumber}
                </a>
              ))}
            </div>
          </div>
          <PackageList packages={result.data} />
        </main>
      ) : result.type === "Started" ? (
        <p>Loading...</p>
      ) : result.type === "Failed" ? (
        <p>Error: {result.error}</p>
      ) : (
        <p />
      )}
    </div>
  )
}

export default SearchResultsPage
